use druid::*;

fn main() {
    let window = WindowDesc::new(ui).title("gcd");

    let data = TwoNumbers {
        a: String::new(),
        b: String::new(),
    };

    AppLauncher::with_window(window)
        .launch(data)
        .expect("to launch window");
}

#[derive(Clone, Lens, Data, Debug)]
struct TwoNumbers {
    a: String,
    b: String,
}

impl TwoNumbers {
    fn gcd(&self) -> Option<i64> {
        let mut r;
        let TwoNumbers { a, b } = self;
        let mut a: i64 = a.parse().ok()?;
        let mut b: i64 = b.parse().ok()?;

        while a % b > 0 {
            r = a % b;
            a = b;
            b = r;
        }

        Some(b)
    }
}

fn ui() -> impl Widget<TwoNumbers> {
    use widget::{Flex, Label, TextBox};
    Flex::column()
        .with_child(
            Flex::row()
                .with_child(TextBox::new().lens(TwoNumbers::a).padding(4.0))
                .with_child(TextBox::new().lens(TwoNumbers::b).padding(4.0)),
        )
        .with_child(Label::new(|data: &TwoNumbers, _: &_| match data.gcd() {
            Some(n) => n.to_string(),
            None => String::new(),
        }))
}
