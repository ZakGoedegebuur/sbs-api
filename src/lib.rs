pub use sbs_api_internal::{ SBI, Serialize, DeSerialize };
pub use sbs_api_macro::{ Serialize, DeSerialize };

#[cfg(test)]
mod tests {

    use crate::{ Serialize, DeSerialize, SBI };

    #[derive(Serialize, DeSerialize, Debug)]
    struct TStruct {
        name: String,
        num: f32,
    }

    #[test]
    fn it_works() {
        let tstruct = TStruct { name: "peetah".to_string(), num: 5634.98 };
        let mut sbi = SBI::new();
        tstruct.serialize(&mut sbi);

        let tstruct = sbi.deserialize::<TStruct>();
        println!("{:#?}", tstruct);
    }
}
