pub type Fl = f32;
pub type Arr = Vec<Fl>;
pub type Mat = Vec<Vec<Fl>>;
pub type FPref = fn(&Fl, &Fl, &Fl) -> Fl;

#[derive(Clone, Debug)]
pub struct MCFlowResult {
    pub pref_matrix_plus_t: Mat,
    pub pref_matrix_minus_t: Mat,
}

#[derive(Clone, Debug)]
pub struct PromResultI {
    pub phi_plus_score: Arr,
    pub phi_minus_score: Arr,
    pub phi_plus_matrix: Mat,
    pub phi_minus_matrix: Mat,
}

#[derive(Clone, Debug)]
pub struct PromResultII {
    pub score: Arr,
    pub normalized_score: Arr,
    pub weighted_flow: Mat,
}

#[derive(Default, Clone, Debug)]
pub struct Criteria {
    pub weight: Arr,
    pub criteria_type: Arr,
    pub pref_function: Vec<String>,
    pub q: Arr,
    pub p: Arr,
}
