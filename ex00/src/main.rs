mod matrix;
mod vector;

use matrix::Matrix;
use vector::Vector;

#[test]
fn random_testcase_vectors() -> std::io::Result<()> {
    use std::io::Write;
    use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

    const NB_TESTCASE_VECTORS: usize = 10;

    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)).set_bold(true))?;
    writeln!(&mut stdout, "TEST WITH VECTORS")?;
    let mut u = Vector::from([2., 3.]);
    u.run_random_tests(NB_TESTCASE_VECTORS);
    stdout.reset()?;
    Ok(())
}

#[test]
fn random_testcase_matrices() -> std::io::Result<()> {
    use std::io::Write;
    use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

    const NB_TESTCASE_MATRICES: usize = 10;

    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Blue)).set_bold(true))?;
    writeln!(&mut stdout, "TEST WITH MATRICES")?;
    let mut u = Matrix::from([[2., 3.], [8.4, 5.4]]);
    u.run_random_tests(NB_TESTCASE_MATRICES);
    stdout.reset()?;
    Ok(())
}

fn main() {
    let mut u = Vector::from([2., 3.]);
    let v = Vector::from([5., 7.]);
    u.add(&v);
    println!("{}", u);
    // [7.0]
    // [10.0]
    let mut u = Vector::from([2., 3.]);
    let v = Vector::from([5., 7.]);
    u.sub(&v);
    println!("{}", u);
    // [-3.0]
    // [-4.0]
    let mut u = Vector::from([2., 3.]);
    u.scl(2.);
    println!("{}", u);
    // [4.0]
    // [6.0]
    let mut u = Matrix::from([[1., 2.], [3., 4.]]);
    let v = Matrix::from([[7., 4.], [-2., 2.]]);
    u.add(&v);
    println!("{}", u);
    // [8.0, 6.0]
    // [1.0, 6.0]
    let mut u = Matrix::from([[1., 2.], [3., 4.]]);
    let v = Matrix::from([[7., 4.], [-2., 2.]]);
    u.sub(&v);
    println!("{}", u);
    // [-6.0, -2.0]
    // [5.0, 2.0]
    let mut u = Matrix::from([[1., 2.], [3., 4.]]);
    u.scl(2.);
    println!("{}", u);
    // [2.0, 4.0]
    // [6.0, 8.0]
}
