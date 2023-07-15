use crate::{declare_eval, declare_params, declare_problem};

declare_params!(coco_futhark::backend::Multicore);
declare_eval!(coco_futhark::backend::Multicore);
declare_problem!(coco_futhark::backend::Multicore);
