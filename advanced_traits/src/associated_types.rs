/*
    Rust has a concept called associated types which are similar to generics, but are used in a different way.
    Associated types are used in traits to define placeholder types that will be used later in the trait.
    While it seems exactly like generics at first,
    the difference is that when using generics, we must annotate the types in each implementation.
    With associated types, we don’t need to annotate types in each implementation.
*/

#[derive(Debug)]
struct Paper;

#[derive(Debug)]
struct Plastic;

trait Printer {
    type Material;
    fn print(&self) -> Self::Material;
}

trait GenericPrinter<T> {
    fn print(&self) -> T;
}

/*
    Here, we defined examples for printers using associated types and generics.
    Let's see how they differ in implementation.
*/

struct ThreeDPrinter;
struct PaperPrinter;

impl Printer for ThreeDPrinter {
    type Material = Plastic;

    fn print(&self) -> Self::Material {
        Plastic {}
    }
}

impl GenericPrinter<Paper> for PaperPrinter {
    fn print(&self) -> Paper {
        Paper {}
    }
}

/*
    Both implementations are straightforward.
    One key difference is that we can still define more implementations for GenericPrinter on PaperPrinter,
    just with different generic types.
    But we can't do that with Printer on ThreeDPrinter, because we already defined the Printer trait on it.
*/

// This would not work!
/* impl Printer for ThreeDPrinter {
    type Material = Paper;

    fn print(&self) -> Self::Material {
        Paper {}
    }
} */

// However, this does work! And suddenly, our PaperPrinter can print plastic!
impl GenericPrinter<Plastic> for PaperPrinter {
    fn print(&self) -> Plastic {
        Plastic {}
    }
}

/*
    Oh also, you can combine generics and associated types!
    This is our new GigaPrinter trait, which can print with a production material and magically result in a different material.
*/
trait GigaPrinter<T> {
    type Material;
    fn print(&self, production_material: T) -> Self::Material;
}

struct PaperToPlasticPrinter;
impl GigaPrinter<Paper> for PaperToPlasticPrinter {
    type Material = Plastic;

    fn print(&self, _production_material: Paper) -> Self::Material {
        Plastic {}
    }
}

/*
    But wait.. This must have some impact on the way we use the different printer instances, right?
    Let's have a look at that!
*/

pub fn run() {
    let three_d_printer = ThreeDPrinter {};
    let paper_printer = PaperPrinter {};
    let paper_to_plastic_printer = PaperToPlasticPrinter {};

    // Using the implementationm with associated types is straightforward
    let plastic = three_d_printer.print();
    println!("Our 3D printer printed plastic: {:?}", plastic);

    // Using the implementation with generics, while allowing to be more flexible, is more verbose
    let paper: Paper = paper_printer.print();
    let flat_plastic: Plastic = paper_printer.print();
    println!(
        "Our paper printer printed paper: {:?} and plastic: {:?}",
        paper, flat_plastic
    );

    // We also do not need to specify types here, as the resulting type of print() is the defined associated type
    let plastic = paper_to_plastic_printer.print(Paper {});
    println!(
        "Our paper to plastic printer printed plastic: {:?}",
        plastic
    );
}
