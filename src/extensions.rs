use periodic_table_on_an_enum::{GroupBlock, StateOfMatter};

pub trait GroupBlockExt {
    fn name(&self) -> &str;
    fn color(&self) -> [u8; 3];
}

impl GroupBlockExt for GroupBlock {
    fn name(&self) -> &str {
        match self {
            GroupBlock::AlkaliMetal => "Alkali Metal",
            GroupBlock::AlkalineEarthMetal => "Alkaline Earth Metal",
            GroupBlock::Lanthanide => "Lanthanide",
            GroupBlock::Actinide => "Actinide",
            GroupBlock::TransitionMetal => "Transition Metal",
            GroupBlock::PostTransitionMetal => "Post Transition Metal",
            GroupBlock::Metalloid => "Metalloid",
            GroupBlock::NonMetal => "Non Metal",
            GroupBlock::Halogen => "Halogen",
            GroupBlock::NobleGas => "Noble Gas",
        }
    }

    fn color(&self) -> [u8; 3] {
        match self {
            GroupBlock::AlkaliMetal => [76, 152, 100],
            GroupBlock::AlkalineEarthMetal => [51, 104, 170],
            GroupBlock::Lanthanide => [214, 77, 148],
            GroupBlock::Actinide => [219, 196, 204],
            GroupBlock::TransitionMetal => [75, 53, 140],
            GroupBlock::PostTransitionMetal => [245, 201, 84],
            GroupBlock::Metalloid => [99, 187, 202],
            GroupBlock::NonMetal => [189, 218, 157],
            GroupBlock::Halogen => [248, 221, 189],
            GroupBlock::NobleGas => [208, 56, 83],
        }
    }
}

pub trait StateOfMatterExt {
    fn name(&self) -> &str;
}

impl StateOfMatterExt for StateOfMatter {
    fn name(&self) -> &str {
        match self {
            StateOfMatter::Solid => "Solid",
            StateOfMatter::Liquid => "Liquid",
            StateOfMatter::Gas => "Gas",
        }
    }
}
