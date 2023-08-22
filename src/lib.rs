varnish::boilerplate!();

use std::error::Error;
use std::num::Wrapping;

use varnish::vcl::ctx::Ctx;
varnish::vtc!(test01);

#[allow(non_camel_case_types)]
pub fn hash<'a>(
    ctx: &mut Ctx,
) -> Result<i64, Box<dyn Error>> {
    let mut hash: Wrapping<u32> = Wrapping(5381);
    for vec in ctx.cached_req_body()? {
        for c in vec {
            hash = ((hash << 5) + hash) + Wrapping(*c as u32);
        }
    }
    return Ok(hash.0 as i64);
}
