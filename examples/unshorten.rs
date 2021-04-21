use core::time::Duration;
use urlexpand;

fn main() {
    // let url = "https://bit.ly/3alqLKi";
    // println!("{}\nis_shortened? {}\nExpanded URL = {:?}",
    //     url,
    //     urlexpand::is_shortened(url),
    //     urlexpand::unshorten(url, Some(Duration::new(30,0)))
    // );
    // println!();
    //
    // let url = "https://bit.ly/3dsTDlV";
    // println!("{}\nis_shortened? {}\nExpanded URL = {:?}",
    //      url,
    //      urlexpand::is_shortened(url),
    //      urlexpand::unshorten(url, Some(Duration::new(30,0)))
    // );
    // println!();
    //
    let url = "https://tinyurl.com/2j582c6a";
    println!("{}\nis_shortened? {}\nExpanded URL = {:?}",
             url,
             urlexpand::is_shortened(url),
             urlexpand::unshorten(url, Some(Duration::new(30,0)))
    );
    println!();

    // let url = "https://t.co/bYeHhy9kAU";
    // println!("{}\nis_shortened? {}\nExpanded URL = {:?}",
    //          url,
    //          urlexpand::is_shortened(url),
    //          urlexpand::unshorten(url, Some(Duration::new(30,0)))
    // );
    // println!();

}