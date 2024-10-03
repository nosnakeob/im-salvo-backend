use proc_macro::TokenStream;
use std::fs;
use std::ops::Add;
use std::path::PathBuf;
use std::str::FromStr;

use syn::visit::Visit;
use syn::Expr::MethodCall;
use syn::Stmt::Expr;
use syn::{parse_file, ItemFn, LitStr};

use crate::utils::path2module_path;
use crate::visitor::route::RocketRouteFnVisitor;

pub fn _rocket_base_path(base_path: LitStr, source_path: PathBuf) -> TokenStream {
    let content = fs::read_to_string(source_path).unwrap();
    let ast = parse_file(&content).unwrap();

    let mut visitor = RocketRouteFnVisitor::new();
    visitor.visit_file(&ast);
    // eprintln!("route_fns: {:?}", visitor.route_fns);

    let route_fns = visitor.route_fns;

    quote!(
        pub fn routes() -> rocket::fairing::AdHoc {
            rocket::fairing::AdHoc::on_ignite(#base_path, |rocket| async {
                rocket.mount(
                    #base_path,
                    routes![ #(#route_fns),* ]
                )
            })
        }

        const BASE: &str = #base_path;
    ).into()
}

pub fn _auto_mount(dir: String, func: &mut ItemFn) {
    if let (Some(Expr(MethodCall(method), _)), Ok(mut entry)) =
        (func.block.stmts.last_mut(), fs::read_dir(&dir)) {
        while let Some(Ok(f)) = entry.next() {
            let route_path = proc_macro2::TokenStream::from_str(path2module_path(&mut f.path()).add("::routes()").as_str()).unwrap();

            *method = parse_quote! { #method
                    .attach(#route_path)
                }
        }
    }
}
