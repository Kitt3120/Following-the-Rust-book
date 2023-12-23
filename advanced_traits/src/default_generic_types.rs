/*
    So far, the concept of generic types should be familiar.
    What hasn't been covered yet is the concept of default generic types.

    When defining a trait with one or multiple generic types,
    default types can be defined for each generic type, in case the type is not specified when implementing the trait.
*/

#[derive(Debug)]
struct Paper;

#[derive(Debug)]
struct Plastic;

trait Printer<Input = Paper, Output = Paper> {
    fn print(&self, input: Input) -> Output;
}

struct PaperPrinter;
struct ThreeDPrinter;
struct PaperToPlasticPrinter;

impl Printer for PaperPrinter {
    fn print(&self, _input: Paper) -> Paper {
        Paper {}
    }
}

impl Printer<Plastic, Plastic> for ThreeDPrinter {
    fn print(&self, _input: Plastic) -> Plastic {
        Plastic {}
    }
}

impl Printer<Paper, Plastic> for PaperToPlasticPrinter {
    fn print(&self, _input: Paper) -> Plastic {
        Plastic {}
    }
}

pub fn run() {
    let paper_printer = PaperPrinter {};
    let three_d_printer = ThreeDPrinter {};
    let paper_to_plastic_printer = PaperToPlasticPrinter {};

    let (paper, more_paper) = (Paper {}, Paper {});
    let plastic = Plastic {};

    let printed_paper = paper_printer.print(paper);
    let printed_plastic = three_d_printer.print(plastic);
    let flat_plastic = paper_to_plastic_printer.print(more_paper);

    println!("Our paper printer printed paper: {:?}", printed_paper);
    println!("Our 3D printer printed plastic: {:?}", printed_plastic);
    println!(
        "Our PaperToPlastic printer printed plastic from paper: {:?}",
        flat_plastic
    );
}
