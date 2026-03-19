
#[test]
fn create_struct() {

    let instance = super::Quadrado {
        lado: 5,
        cor: String::from("vermelho")
    };

}

#[test]
fn test_constructor() {
    let _struct = super::Quadrado::new(5, String::from("vermelho"));
}