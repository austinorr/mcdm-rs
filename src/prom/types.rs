use ndarray::{Array2, Axis};

pub type Fl = f32;
pub type Arr = Vec<Fl>;
pub type Mat = Vec<Arr>;
pub type FPref = fn(&Fl, &Fl, &Fl) -> Fl;
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub trait FromVec2 {
    fn from_vec2(vec2: Mat) -> Array2<Fl>;
}

impl FromVec2 for Array2<Fl> {
    fn from_vec2(vec2: Mat) -> Self {
        let r = vec2.len();
        let c = vec2[0].len();

        let mut arr = Array2::<Fl>::default((r, c));
        for (i, mut row) in arr.axis_iter_mut(Axis(0)).enumerate() {
            for (j, col) in row.iter_mut().enumerate() {
                *col = vec2[i][j];
            }
        }
        arr
    }
}
