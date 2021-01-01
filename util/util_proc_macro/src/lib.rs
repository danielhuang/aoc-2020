use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_attribute]
pub fn bench(_attr: TokenStream, item: TokenStream) -> TokenStream {
	let item: proc_macro2::TokenStream = item.into();
	(quote! {
		fn main() {
			let bench = ::std::env::var("AOC_BENCHMARK").is_ok();
			if bench {
				::util::run_benchmark(|| {
					format!("{:?}", main())
				}, file!());
			} else {
				println!("{:?}", main());
			}
			#item
		}
	})
	.into()
}
