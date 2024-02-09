fn main() -> anyhow::Result<()> {
    let mut num = String::new();
    std::io::stdin().read_line(&mut num)?;
    let num = num.trim();

    let val = (num.parse::<f32>()? as f32 * 2.3 + 6.0) / 2.0;
    println!("val is {}", val);

    Ok(())
}