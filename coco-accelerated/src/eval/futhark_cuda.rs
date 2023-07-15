use crate::{declare_eval, declare_params, declare_problem};

declare_params!(coco_futhark::backend::Cuda);
declare_eval!(coco_futhark::backend::Cuda);
declare_problem!(coco_futhark::backend::Cuda);
