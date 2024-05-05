use std::io::{self, Write};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

mod matrix;
mod vector;

use matrix::Matrix;
use vector::Vector;

const NB_TESTCASE_VECTORS: usize = 10;
const NB_TESTCASE_MATRICES: usize = 10;

fn random_testcase_vectors(nb_testcase: usize) {
    let mut u = Vector::from([2., 3.]);
    u.run_random_tests(nb_testcase);
}

fn random_testcase_matrices(nb_testcase: usize) {
    let mut u = Matrix::from([[2., 3.], [8.4, 5.4]]);
    u.run_random_tests(nb_testcase);
}

fn main() -> io::Result<()> {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)).set_bold(true))?;
    writeln!(&mut stdout, "TEST WITH VECTORS")?;
    random_testcase_vectors(NB_TESTCASE_VECTORS);
    stdout.reset()?;
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Blue)).set_bold(true))?;
    writeln!(&mut stdout, "TEST WITH MATRICES")?;
    random_testcase_matrices(NB_TESTCASE_MATRICES);
    Ok(())
}
