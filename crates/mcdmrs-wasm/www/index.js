import init, {
  initThreadPool,
  PromJS,
  init_panic_hook,
} from "./pkg/mcdmrs_wasm.js";

let p; // global prom instance
let weights = []; // user interaction
let loaded_criteria; // loaded by user
let loaded_data;
let matrix_t;
let data_columns;

function index_max(array, n) {
  let sorted_arr = array.toSorted((a, b) => b - a).slice(0, n);
  let _ix = [];
  for (const i in array) {
    if (sorted_arr.includes(array[i])) {
      _ix.push({ ix: i, val: array[i] });
    }
  }

  return _ix.sort((a, b) => b.val - a.val).map((v) => v.ix);
}

async function input_summary() {
  let loaded_data_notes = Object.assign(document.createElement("div"), {
    style: "margin: 1em 0 1em 0",
    id: "loaded-data-notes",
  });
  loaded_data_notes.innerText = `${loaded_data.length.toLocaleString()} records loaded. `;

  let n_comparisions =
    matrix_t.length * matrix_t[0].length * matrix_t[0].length;
  loaded_data_notes.innerText += `Computing ${n_comparisions.toLocaleString()} pairwise comparisons.`;

  let data_summary = document.getElementById("data_summary");
  data_summary.innerHTML = makeTableHTML(
    loaded_criteria.map((row) => row.slice(1))
  );
  data_summary.append(loaded_data_notes);
}

function makeTableHTML(myArray) {
  let result = "<table>";
  for (const i in myArray) {
    result += "<tr>";
    for (const j in myArray[i]) {
      result += "<td>" + myArray[i][j] + "</td>";
    }
    result += "</tr>";
  }
  result += "</table>";

  return result;
}

async function weight_sliders() {
  weights = [...Array(matrix_t.length)].map((_) => 1);
  let sliders = document.getElementById("slidecontainer");
  sliders.innerHTML = "";

  for (const i in matrix_t) {
    let group = Object.assign(document.createElement("div"), {
      style: "display: flex; align-content: center; padding: 0.5em;",
    });

    let label = Object.assign(document.createElement("label"), {
      id: `weight_l_${i}}`,
      for: `weight_i_${i}`,
      style: "display: inline-block; width: 200px; justify-content:center",
    });
    let value = Object.assign(document.createElement("span"), {
      id: `weight_v_${i}}`,
      // style:
      //   "display: inline-block; width: 200px; justify-content:center; align-content:right;",
    });
    value.setAttribute("id", `weight_v_${i}}`);
    let slider = Object.assign(document.createElement("input"), {
      id: `weight_i_${i}`,
      name: `weight_i_${i}`,
      type: "range",
      min: "0.1",
      max: "5",
      step: 0.1,
      class: "slider",
      data_index: `${i}`,
    });

    slider.value = "1";
    let colname = data_columns[parseInt(i) + 1];
    label.innerHTML = colname; //`${colname} ${slider.value}`;
    value.innerHTML = `${slider.value}`;

    slider.oninput = function () {
      let ix = parseInt(this.data_index);
      value.innerHTML = `${this.value}`;
      weights[ix] = parseFloat(this.value);

      re_calculate();
    };

    slider.addEventListener("wheel", function (e) {
      if (e.deltaY < 0) {
        this.value = (
          parseFloat(this.value) + parseFloat(this.step)
        ).toString();
      } else {
        this.value = (
          parseFloat(this.value) - parseFloat(this.step)
        ).toString();
      }
      let ix = parseInt(this.data_index);
      value.innerHTML = `${this.value}`;
      weights[ix] = parseFloat(this.value);

      re_calculate();

      e.preventDefault();
      e.stopPropagation();
    });

    group.append(label, slider, value);
    sliders.append(group);
  }
}

export async function load_data(n_alts = 10) {
  const { criteria, data_long, transpose } = await import("./js/data.js");
  data_columns = data_long[0].map((s) =>
    s
      .trim()
      .replaceAll("_", " ")
      .split(" ")
      .map((w) => w.charAt(0).toUpperCase() + w.slice(1))
      .join(" ")
  );

  loaded_criteria = criteria;

  loaded_data = data_long
    .slice(0, n_alts + 1)
    .filter(
      (e) =>
        e.length > 1 &&
        e.every(
          (e) => !isNaN(parseFloat(e)) && typeof parseFloat(e) === "number"
        )
    );

  matrix_t = transpose(loaded_data.map((row) => row.slice(1))); // slice off index column column
  // matrix_t.pop();
  let cr = transpose(loaded_criteria)
    .slice(2)
    .map((row) => row.slice(1));

  try {
    p = new PromJS(matrix_t.flat(), matrix_t[0].length, matrix_t.length, ...cr);
  } catch (e) {
    alert(e);
    return;
  }

  input_summary();
  weight_sliders();
}

async function re_calculate() {
  let start, end;

  start = performance.now();
  p.compute_prom_ii();
  end = performance.now();
  console.log(`ms: ${(end - start).toPrecision(4)}`);

  start = performance.now();
  p.re_weight(weights);
  end = performance.now();
  console.log(`ms: ${(end - start).toPrecision(4)}`);

  let score = p.get_score();
  let best_rows = index_max(score, 10);

  let brief = [[...data_columns, "Score"]];
  for (const i of best_rows) {
    let row = [...loaded_data[i]];
    row.push(score[i]);
    brief.push(
      row.map((v, i) =>
        i === 0
          ? parseInt(v)
          : parseFloat(v).toLocaleString("en", {
              maximumFractionDigits: 2,
            })
      )
    );
  }

  let results = document.getElementById("results_table");
  results.innerHTML = "";
  results.innerHTML = makeTableHTML(brief);
}

export async function main() {
  await init();
  await initThreadPool(
    Math.max(Math.ceil(navigator.hardwareConcurrency / 2), 2)
  );

  init_panic_hook();

  let mc_flow = document.getElementById("compute_prom_ii");
  mc_flow.addEventListener("click", async () => {
    if (!p) return;
    await re_calculate();
  });
}
