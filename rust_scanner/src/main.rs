
use rayon::prelude::*;

fn main() -> Result<()> {
    //we use a custom thread pool to improve speed
    let pool = rayon::ThreadPoolBuilder::new().num_threads(256).build().unwrap();
    
    //pool.install is required to use our custom threadpool, instead of 
    // rayon's default one

    pool.install(|| {
        let scan_result : Vec<Subdomain> =
        subdomains::enumerate(&http_client , target).unwrap().into_par_iter().map(subdomains::scan_ports).collect();

        for subdomain in scan_result {
            println!("{}", &subdomain.domain);
            for port in &subdomain.open_ports {
                println!("         {}", port.port);
            }
            println!("");
        }

    });   
}
