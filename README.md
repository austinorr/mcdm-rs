# `mcdmrs`

High performance Multi-Criterion Decision Making Algorithms (MCDMs) written in Rust for 🚀.

Currently includes Promethee I/II, with room to expand.

## Project Features

**Parallelism** This project leverages `rayon` for to take advantage of every core.

**Low-Memory** These algorithms often require pair-wise comparisons which can rapidly exceed the memory available if care is not taken. In the **Usage** section below we analyze 10k alternatives, which requires over 100,000,000 comparisons **per criteria** and there are 7 criteria.

**High Performance** The combination of the above features means that this library is very high performance. On the test machine the **Usage** example below runs in under 300ms and with a max memory usage of ~17MB.

**Optional Dependencies** When used as a library this project depends only on `rayon`. For command line usage and csv IO the feature flags of `cli` and `io` can be enabled. The project uses the `clap` and `polars` crates for these features, respectively.

**Testing, Benching, Coverage** This project is tested against known-correct input and outputs from a python-based reference implementation produced by this research paper: [pymcdm—The universal library for solving multi-criteria decision-making problems](https://www.sciencedirect.com/science/article/pii/S235271102300064X)

It also has near-complete test coverage for all functions and traits (use: `make coverage`), and performance benchmarks for regression testing.

## Usage

```bash
$cargo run -- --help

Usage: mcdmrs --alternatives <ALTERNATIVES> --criteria <CRITERIA>

Options:
  -a, --alternatives <ALTERNATIVES>  The path to the alternatives file
  -c, --criteria <CRITERIA>          The path to the criteria file
  -h, --help                         Print help
  -V, --version                      Print version

$cargo run -- -a ./examples/data/alternatives_long.csv -c ./examples/data/criteria.csv
...
Calculation time (ms):  280.03
Scoring Criteria shape: (7, 7)
╭─────┬────────────────────────┬────────┬───────────────┬───────────────┬─────────┬────────╮
│     ┆ name                   ┆ weight ┆ criteria_type ┆ pref_function ┆ q       ┆ p      │
│ --- ┆ ---                    ┆ ---    ┆ ---           ┆ ---           ┆ ---     ┆ ---    │
│ i64 ┆ str                    ┆ i64    ┆ i64           ┆ str           ┆ f64     ┆ i64    │
╞═════╪════════════════════════╪════════╪═══════════════╪═══════════════╪═════════╪════════╡
│ 0   ┆ cost                   ┆ 2      ┆ -1            ┆ linear        ┆ 20000.0 ┆ 100000 │
│ 1   ┆ treated_area           ┆ 2      ┆ 1             ┆ linear        ┆ 3.0     ┆ 10     │
│ 2   ┆ site_slope             ┆ 1      ┆ -1            ┆ ushape        ┆ 0.03    ┆ 0      │
│ 3   ┆ site_footprint         ┆ 1      ┆ -1            ┆ ushape        ┆ 500.0   ┆ 0      │
│ 4   ┆ tss_conc_pct_reduction ┆ 1      ┆ 1             ┆ usual         ┆ 0.0     ┆ 0      │
│ 5   ┆ site_inequity_factor   ┆ 2      ┆ -1            ┆ usual         ┆ 0.0     ┆ 0      │
│ 6   ┆ risk_factor            ┆ 1      ┆ -1            ┆ usual         ┆ 0.0     ┆ 0      │
╰─────┴────────────────────────┴────────┴───────────────┴───────────────┴─────────┴────────╯
Data with Prom II Scores shape: (10_000, 10)
╭──────┬───────────┬──────────────┬────────────┬────────────────┬────────────────────────┬────────────────────┬─────────────┬───────────┬──────────────────╮
│      ┆ cost      ┆ treated_area ┆ site_slope ┆ site_footprint ┆ tss_conc_pct_reduction ┆ site_equity_factor ┆ risk_factor ┆ score     ┆ normalized_score │
│ ---  ┆ ---       ┆ ---          ┆ ---        ┆ ---            ┆ ---                    ┆ ---                ┆ ---         ┆ ---       ┆ ---              │
│ i64  ┆ f64       ┆ f64          ┆ f64        ┆ f64            ┆ f64                    ┆ f64                ┆ i64         ┆ f32       ┆ f32              │
╞══════╪═══════════╪══════════════╪════════════╪════════════════╪════════════════════════╪════════════════════╪═════════════╪═══════════╪══════════════════╡
│ 5318 ┆ 117113.73 ┆ 378.7        ┆ 0.009178   ┆ 3423.120235    ┆ 82.152996              ┆ -0.863069          ┆ 0           ┆ 0.709592  ┆ 1.0              │
│ 347  ┆ 50806.73  ┆ 463.7        ┆ 0.009909   ┆ 113815.334284  ┆ 71.402867              ┆ -0.914706          ┆ 0           ┆ 0.693671  ┆ 0.988785         │
│ 31   ┆ 135956.4  ┆ 492.4        ┆ 0.009887   ┆ 19882.717937   ┆ 77.494967              ┆ -0.451588          ┆ 0           ┆ 0.685754  ┆ 0.983208         │
│ 4630 ┆ 101568.0  ┆ 439.7        ┆ 0.033597   ┆ 8248.914187    ┆ 90.814973              ┆ -0.854272          ┆ 1           ┆ 0.680243  ┆ 0.979326         │
│ 3622 ┆ 168381.12 ┆ 422.6        ┆ 0.029871   ┆ 800.499515     ┆ 70.834023              ┆ -0.909014          ┆ 0           ┆ 0.67318   ┆ 0.974351         │
│ 9491 ┆ 49510.08  ┆ 490.8        ┆ 0.058577   ┆ 50337.557767   ┆ 69.73083               ┆ -0.846157          ┆ 0           ┆ 0.669598  ┆ 0.971827         │
│ 9191 ┆ 30637.44  ┆ 492.7        ┆ 0.032984   ┆ 96345.542341   ┆ 56.80086               ┆ -0.778482          ┆ 0           ┆ 0.645874  ┆ 0.955115         │
│ 8624 ┆ 33873.1   ┆ 479.8        ┆ 0.064361   ┆ 6800.084921    ┆ 44.878234              ┆ -0.835063          ┆ 0           ┆ 0.639289  ┆ 0.950476         │
│ 2171 ┆ 67355.98  ┆ 439.7        ┆ 0.039653   ┆ 72481.927819   ┆ 60.994916              ┆ -0.917286          ┆ 0           ┆ 0.631372  ┆ 0.9449           │
│ 6843 ┆ 78077.77  ┆ 372.5        ┆ 0.004476   ┆ 43011.886287   ┆ 84.624707              ┆ -0.82791           ┆ 1           ┆ 0.626468  ┆ 0.941445         │
│ …    ┆ …         ┆ …            ┆ …          ┆ …              ┆ …                      ┆ …                  ┆ …           ┆ …         ┆ …                │
│ 5439 ┆ 593792.75 ┆ 11.7         ┆ 0.048334   ┆ 206851.101955  ┆ 10.217043              ┆ 0.504018           ┆ 1           ┆ -0.626651 ┆ 0.058707         │
│ 6005 ┆ 674389.88 ┆ 111.2        ┆ 0.062734   ┆ 116930.476684  ┆ 17.763635              ┆ 0.809555           ┆ 2           ┆ -0.634819 ┆ 0.052954         │
│ 5811 ┆ 690600.22 ┆ 26.9         ┆ 0.070593   ┆ 25546.477609   ┆ 16.852872              ┆ 0.83388            ┆ 2           ┆ -0.638754 ┆ 0.050181         │
│ 1029 ┆ 583662.52 ┆ 7.9          ┆ 0.055498   ┆ 196569.126882  ┆ 10.92466               ┆ 0.275603           ┆ 2           ┆ -0.645025 ┆ 0.045764         │
│ 6312 ┆ 476094.58 ┆ 18.3         ┆ 0.078774   ┆ 147470.354821  ┆ 27.206456              ┆ 0.930839           ┆ 2           ┆ -0.653355 ┆ 0.039896         │
│ 6851 ┆ 653413.46 ┆ 8.8          ┆ 0.024579   ┆ 217600.370686  ┆ 28.031312              ┆ 0.979804           ┆ 1           ┆ -0.670783 ┆ 0.027619         │
│ 2247 ┆ 585333.99 ┆ 52.4         ┆ 0.06587    ┆ 152036.296085  ┆ 19.607451              ┆ 0.83984            ┆ 2           ┆ -0.678169 ┆ 0.022416         │
│ 3475 ┆ 565911.44 ┆ 13.6         ┆ 0.069494   ┆ 172055.455049  ┆ 39.847876              ┆ 0.908325           ┆ 2           ┆ -0.684984 ┆ 0.017616         │
│ 715  ┆ 652079.82 ┆ 19.0         ┆ 0.059757   ┆ 169871.591941  ┆ 17.761949              ┆ 0.610461           ┆ 2           ┆ -0.703265 ┆ 0.004738         │
│ 9551 ┆ 663459.86 ┆ 5.6          ┆ 0.048246   ┆ 140070.325274  ┆ 10.999873              ┆ 0.714187           ┆ 2           ┆ -0.709991 ┆ 0.0              │
╰──────┴───────────┴──────────────┴────────────┴────────────────┴────────────────────────┴────────────────────┴─────────────┴───────────┴──────────────────╯
```

## To Do:

- This project has implemented it's own matrix and array operations. Future plans include to leverage powerful crates such as `ndarray` and `polars` for array operations. It's unlikely that this will yield much performance benefit though, since these operations are not on the 'hot path' of the algorithm. These custom operations include:

  - 2D matrix transpose (`prom::math::transpose`)
  - (1D, 1D) => 1D element-wise difference (`prom::math::diff`)
  - 2D \* 1D => 2D broadcasted multiplication (`prom::math::mult_axis_0`)
  - 2D => 1D reduce by sum (`prom::math::sum_axis_0`)

  Each of these uses iterators to be as efficient as possible.

- Increase library flexibility by implementing more generic functions.
