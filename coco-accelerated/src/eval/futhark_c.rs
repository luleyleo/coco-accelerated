use crate::{declare_eval, declare_params, declare_problem};

declare_params!(coco_futhark::backend::C);
declare_eval!(coco_futhark::backend::C);
declare_problem!(coco_futhark::backend::C);
