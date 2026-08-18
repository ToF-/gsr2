
fn main() {
    let src = r#"
graph LR
    D(Dispatcher) -- has --> C(GsrController) -- has --> A(GsrApplicationWindow)
    A -- activates action --> D
"#;

    let output = mermaid_text::render(src).unwrap();
    println!("{output}");
}
