use serenum::SerEnum;

#[test]
fn test_macro() {
    #[derive(SerEnum)]
    pub enum Order {
        #[serenum(text = "full")]
        Full,
        #[serenum(text = "short")]
        Short,
    }

    assert_eq!(Order::Full.text(), "full");
    assert_eq!(Order::Short.text(), "short");
}