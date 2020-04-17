use proc_macro::{Delimiter, TokenStream, TokenTree};

fn gen_clean(gen: &String) -> String {
    if gen.is_empty() {
        return String::new();
    }
    let mut gen = gen
        .split_terminator(',')
        .map(|g| g.split_terminator(':').nth(0).unwrap())
        .collect::<Vec<_>>()
        .join(",");
    if !gen.contains('>') {
        gen.push('>');
    }
    gen
}

fn struct_fields(input: TokenStream) -> Vec<String> {
    let mut fields = Vec::new();
    let mut field = String::new();
    let mut colon = 0;
    for node in input {
        match node {
            TokenTree::Punct(punct) => {
                if punct.as_char() == ':' {
                    colon += 1;
                }
            }
            _ => {
                if colon == 1 {
                    fields.push(field);
                }
                field = node.to_string();
                colon = 0;
            }
        }
    }
    fields
}

fn struct_ret(name: String, gen: String, whr: String, fields: Vec<String>) -> TokenStream {
    if name.is_empty() {
        panic!("structure name not found");
    }
    if fields.is_empty() {
        panic!("structure field not found");
    }
    let mut p1 = String::new();
    let mut p2 = String::new();
    for ref f in fields {
        p1.push_str("self.");
        p1.push_str(f);
        p1.push_str(".store(__q);");
        p2.push_str(f);
        p2.push_str(":Store");
        p2.push_str("::restore(__q),");
    }
    format!(
        "
        impl {} Store for {} {} {} {{
            fn store(&self, __q: &mut ByteQue) {{ {} }}
            fn restore(__q: &mut ByteQue) -> Self {{ {} {{ {} }} }}
        }}
        ",
        gen,
        name,
        gen_clean(&gen),
        whr,
        p1,
        name,
        p2
    )
    .parse()
    .unwrap()
}

fn tuple_struct_ret(name: String, gen: String, whr: String, field_count: usize) -> TokenStream {
    if name.is_empty() {
        panic!("structure name not found");
    }
    if field_count == 0 {
        panic!("cannot be without fields");
    }
    let mut p1 = String::new();
    let mut p2 = String::new();
    for i in 0..field_count {
        p1.push_str("self.");
        p1.push_str(&i.to_string());
        p1.push_str(".store(__q);");
        p2.push_str("Store");
        p2.push_str("::restore(__q),");
    }
    format!(
        "
        impl {} Store for {} {} {} {{
            fn store(&self, __q: &mut ByteQue) {{ {} }}
            fn restore(__q: &mut ByteQue) -> Self {{ {} ( {} ) }}
        }}
        ",
        gen,
        name,
        gen_clean(&gen),
        whr,
        p1,
        name,
        p2
    )
    .parse()
    .unwrap()
}

fn enum_fields(input: TokenStream) -> Vec<(String, usize, Vec<String>)> {
    let mut fields = Vec::new();
    let mut field = (String::new(), 0, Vec::new());
    let mut extra = false;
    for node in input {
        match node {
            TokenTree::Ident(ident) => {
                field.0 = ident.to_string();
                extra = true;
            }
            TokenTree::Punct(punct) => {
                if punct.as_char() == ',' {
                    fields.push(field);
                    extra = false;
                    field = (String::new(), 0, Vec::new());
                }
            }
            TokenTree::Group(group) => match group.delimiter() {
                Delimiter::Parenthesis => field.1 = group.to_string().split_terminator(',').count(),
                Delimiter::Brace => field.2 = struct_fields(group.stream()),
                _ => (),
            },
            _ => (),
        }
    }
    if extra {
        fields.push(field);
    }
    fields
}

fn enum_ret(
    name: String,
    gen: String,
    whr: String,
    fields: Vec<(String, usize, Vec<String>)>,
) -> TokenStream {
    if name.is_empty() {
        panic!("enum name cannot be found");
    }
    if fields.is_empty() {
        panic!("enum fields cannot be found");
    }
    let mut p1 = String::new();
    let mut p2 = String::new();
    for i in 0..fields.len() {
        if !fields[i].2.is_empty() {
            p1.push_str(&name);
            p1.push_str("::");
            p1.push_str(&fields[i].0);
            p1.push('{');
            for f in &fields[i].2 {
                p1.push_str(f);
                p1.push(',');
            }
            p1.push_str("}=>{");
            p1.push_str(&i.to_string());
            p1.push_str("usize.store(__q);");
            for f in &fields[i].2 {
                p1.push_str(f);
                p1.push_str(".store(__q);");
            }
            p1.push('}');
            if i < fields.len() - 1 {
                p2.push_str(&i.to_string());
            } else {
                p2.push('_');
            }
            p2.push_str("=>");
            p2.push_str(&name);
            p2.push_str("::");
            p2.push_str(&fields[i].0);
            p2.push('{');
            for f in &fields[i].2 {
                p2.push_str(f);
                p2.push(':');
                p2.push_str("Store::restore(__q),");
            }
            p2.push_str("},");
        } else if fields[i].1 > 0 {
            p1.push_str(&name);
            p1.push_str("::");
            p1.push_str(&fields[i].0);
            p1.push('(');
            for j in 0..fields[i].1 {
                p1.push_str("__");
                p1.push_str(&j.to_string());
                p1.push(',');
            }
            p1.push_str(")=>{");
            p1.push_str(&i.to_string());
            p1.push_str("usize.store(__q);");
            for j in 0..fields[i].1 {
                p1.push_str("__");
                p1.push_str(&j.to_string());
                p1.push_str(".store(__q);");
            }
            p1.push('}');
            if i < fields.len() - 1 {
                p2.push_str(&i.to_string());
            } else {
                p2.push('_');
            }
            p2.push_str("=>");
            p2.push_str(&name);
            p2.push_str("::");
            p2.push_str(&fields[i].0);
            p2.push('(');
            for _ in 0..fields[i].1 {
                p2.push_str("Store::restore(__q),");
            }
            p2.push_str("),");
        } else {
            p1.push_str(&name);
            p1.push_str("::");
            p1.push_str(&fields[i].0);
            p1.push_str("=>");
            p1.push_str(&i.to_string());
            p1.push_str("usize.store(__q),");
            if i < fields.len() - 1 {
                p2.push_str(&i.to_string());
            } else {
                p2.push('_');
            }
            p2.push_str("=>");
            p2.push_str(&name);
            p2.push_str("::");
            p2.push_str(&fields[i].0);
            p2.push(',');
        }
    }
    format!(
        "
        impl {} Store for {} {} {} {{
            fn store(&self, __q: &mut ByteQue) {{ match self {{ {} }} }}
            fn restore(__q: &mut ByteQue) -> Self {{ match usize::restore(__q) {{ {} }} }}
        }}
        ",
        gen,
        name,
        gen_clean(&gen),
        whr,
        p1,
        p2
    )
    .parse()
    .unwrap()
}

pub(super) fn common_store(input: TokenStream) -> TokenStream {
    let mut is_struct = false;
    let mut is_enum = false;
    let mut name = String::new();
    let mut is_gen = false;
    let mut gen = String::new();
    let mut whr = String::new();
    for node in input {
        match node {
            TokenTree::Ident(ident) => {
                let ident = ident.to_string();
                if ident == "struct" {
                    is_struct = true;
                    continue;
                }
                if ident == "enum" {
                    is_enum = true;
                    continue;
                }
                if is_struct || is_enum {
                    if is_gen {
                        if ident == "where" {
                            whr.push_str("where ");
                        } else if whr.len() > 0 {
                            whr.push_str(&ident);
                        } else {
                            gen.push_str(&ident);
                        }
                    } else {
                        name = ident;
                    }
                }
            }
            TokenTree::Punct(punct) => {
                if is_struct || is_enum {
                    let punct = punct.as_char();
                    if punct == '<' {
                        is_gen = true;
                    }
                    if is_gen {
                        if whr.len() > 0 {
                            whr.push(punct);
                        } else {
                            gen.push(punct);
                        }
                    }
                }
            }
            TokenTree::Group(group) => {
                if is_struct || is_enum {
                    match group.delimiter() {
                        Delimiter::Parenthesis => {
                            if is_struct {
                                return tuple_struct_ret(
                                    name,
                                    gen,
                                    whr,
                                    group.to_string().split_terminator(',').count(),
                                );
                            }
                        }
                        Delimiter::Brace => {
                            if is_struct {
                                return struct_ret(name, gen, whr, struct_fields(group.stream()));
                            }
                            return enum_ret(name, gen, whr, enum_fields(group.stream()));
                        }
                        _ => (),
                    }
                }
            }
            _ => (),
        }
    }
    panic!("can only be usual structures or enums")
}
