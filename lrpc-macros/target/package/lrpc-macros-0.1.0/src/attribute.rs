use proc_macro::{Delimiter, TokenStream, TokenTree};

fn fun_ret(vis: String, name: String, args: String, body: String, ret: String) -> TokenStream {
    let mut slf = String::new();
    let mut exp = String::new();
    for a in args.split_terminator(',') {
        if a.contains("self") {
            slf = a.to_string() + ", ";
            continue;
        }
        exp.push_str(
            &format!(
                "
                    if __q.len()==0 {{
                        true.store(&mut __r);
                        String::from(\"error when calling function {0} to restore parameters to {1}\").store(&mut __r);
                        return __r;
                    }}
                    let {1}=Store::restore(__q);
                ", name, a
            )
        );
    }
    let rst = if ret.contains("Result") {
        "
            match __s {
                Ok(__t) => {
                    false.store(&mut __r);
                    __t.store(&mut __r);
                }
                Err(__e) => {
                    true.store(&mut __r);
                    format!(\"{}\",__e).store(&mut __r);
                }
            }
        "
    } else {
        "
            false.store(&mut __r);
            __s.store(&mut __r);
        "
    }
    .to_string();
    format!(
        "
        {} fn {}({}__q: &mut ByteQue) -> ByteQue {{
            let mut __r=ByteQue::new();
            {}
            if __q.len()!=0 {{
                true.store(&mut __r);
                String::from(\"error when calling function {} to restore parameters\").store(&mut __r);
                return __r;
            }}
            let __s=(||{} {})();
            {}
            __r
        }}
        ",
        vis, name, slf, exp, name, ret, body, rst
    )
    .parse()
    .unwrap()
}

pub(super) fn fmt_function(input: TokenStream) -> TokenStream {
    let mut is_fn = false;
    let mut is_arg = false;
    let mut vis = String::new();
    let mut name = String::new();
    let mut args = String::new();
    let mut ret = String::new();
    for node in input {
        match node {
            TokenTree::Ident(ident) => {
                let ident = ident.to_string();
                if ident == "fn" {
                    is_fn = true;
                    continue;
                }
                if is_fn {
                    if is_arg {
                        ret.push_str(&ident);
                    } else {
                        name = ident;
                    }
                } else {
                    vis.push_str(&ident);
                }
            }
            TokenTree::Punct(punct) => {
                if is_fn {
                    let punct = punct.as_char();
                    if is_arg {
                        ret.push(punct);
                    } else if punct == '<' {
                        panic!("cannot be a generic function");
                    }
                }
            }
            TokenTree::Group(group) => {
                if is_fn {
                    match group.delimiter() {
                        Delimiter::Parenthesis => {
                            if is_arg {
                                ret.push_str(&group.to_string());
                            } else {
                                is_arg = true;
                                args = group.stream().to_string();
                            }
                        }
                        Delimiter::Brace => {
                            return fun_ret(vis, name, args, group.to_string(), ret);
                        }
                        _ => (),
                    }
                } else {
                    vis.push_str(&group.to_string());
                }
            }
            _ => (),
        }
    }
    panic!("can only be usual function")
}
