export fold;
export fold_crate, fold_mod, fold_fn;
export default_seq_fold;
export default_seq_fold_crate;
export default_seq_fold_mod;
export default_seq_fold_fn;
export default_seq_fold_const;
export default_seq_fold_enum;
export default_seq_fold_res;
export default_seq_fold_iface;
export default_seq_fold_impl;
export default_seq_fold_type;

enum fold<T> = t<T>;

type fold_crate<T> = fn~(fold: fold<T>, doc: doc::cratedoc) -> doc::cratedoc;
type fold_mod<T> = fn~(fold: fold<T>, doc: doc::moddoc) -> doc::moddoc;
type fold_fn<T> = fn~(fold: fold<T>, doc: doc::fndoc) -> doc::fndoc;
type fold_const<T> = fn~(fold: fold<T>, doc: doc::constdoc) -> doc::constdoc;
type fold_enum<T> = fn~(fold: fold<T>, doc: doc::enumdoc) -> doc::enumdoc;
type fold_res<T> = fn~(fold: fold<T>, doc: doc::resdoc) -> doc::resdoc;
type fold_iface<T> = fn~(fold: fold<T>, doc: doc::ifacedoc) -> doc::ifacedoc;
type fold_impl<T> = fn~(fold: fold<T>, doc: doc::impldoc) -> doc::impldoc;
type fold_type<T> = fn~(fold: fold<T>, doc: doc::tydoc) -> doc::tydoc;

type t<T> = {
    ctxt: T,
    fold_crate: fold_crate<T>,
    fold_mod: fold_mod<T>,
    fold_fn: fold_fn<T>,
    fold_const: fold_const<T>,
    fold_enum: fold_enum<T>,
    fold_res: fold_res<T>,
    fold_iface: fold_iface<T>,
    fold_impl: fold_impl<T>,
    fold_type: fold_type<T>
};


// This exists because fn types don't infer correctly as record
// initializers, but they do as function arguments
fn mk_fold<T:copy>(
    ctxt: T,
    fold_crate: fold_crate<T>,
    fold_mod: fold_mod<T>,
    fold_fn: fold_fn<T>,
    fold_const: fold_const<T>,
    fold_enum: fold_enum<T>,
    fold_res: fold_res<T>,
    fold_iface: fold_iface<T>,
    fold_impl: fold_impl<T>,
    fold_type: fold_type<T>
) -> fold<T> {
    fold({
        ctxt: ctxt,
        fold_crate: fold_crate,
        fold_mod: fold_mod,
        fold_fn: fold_fn,
        fold_const: fold_const,
        fold_enum: fold_enum,
        fold_res: fold_res,
        fold_iface: fold_iface,
        fold_impl: fold_impl,
        fold_type: fold_type
    })
}

fn default_seq_fold<T:copy>(ctxt: T) -> fold<T> {
    mk_fold(
        ctxt,
        {|f, d| default_seq_fold_crate(f, d)},
        {|f, d| default_seq_fold_mod(f, d)},
        {|f, d| default_seq_fold_fn(f, d)},
        {|f, d| default_seq_fold_const(f, d)},
        {|f, d| default_seq_fold_enum(f, d)},
        {|f, d| default_seq_fold_res(f, d)},
        {|f, d| default_seq_fold_iface(f, d)},
        {|f, d| default_seq_fold_impl(f, d)},
        {|f, d| default_seq_fold_type(f, d)}
    )
}

fn default_seq_fold_crate<T>(
    fold: fold<T>,
    doc: doc::cratedoc
) -> doc::cratedoc {
    {
        topmod: fold.fold_mod(fold, doc.topmod)
    }
}

fn default_seq_fold_mod<T>(
    fold: fold<T>,
    doc: doc::moddoc
) -> doc::moddoc {
    {
        items: ~vec::map(*doc.items) {|itemtag|
            alt itemtag {
              doc::modtag(moddoc) {
                doc::modtag(fold.fold_mod(fold, moddoc))
              }
              doc::fntag(fndoc) {
                doc::fntag(fold.fold_fn(fold, fndoc))
              }
              doc::consttag(constdoc) {
                doc::consttag(fold.fold_const(fold, constdoc))
              }
              doc::enumtag(enumdoc) {
                doc::enumtag(fold.fold_enum(fold, enumdoc))
              }
              doc::restag(resdoc) {
                doc::restag(fold.fold_res(fold, resdoc))
              }
              doc::ifacetag(ifacedoc) {
                doc::ifacetag(fold.fold_iface(fold, ifacedoc))
              }
              doc::impltag(impldoc) {
                doc::impltag(fold.fold_impl(fold, impldoc))
              }
              doc::tytag(tydoc) {
                doc::tytag(fold.fold_type(fold, tydoc))
              }
            }
        }
        with doc
    }
}

fn default_seq_fold_fn<T>(
    _fold: fold<T>,
    doc: doc::fndoc
) -> doc::fndoc {
    doc
}

fn default_seq_fold_const<T>(
    _fold: fold<T>,
    doc: doc::constdoc
) -> doc::constdoc {
    doc
}

fn default_seq_fold_enum<T>(
    _fold: fold<T>,
    doc: doc::enumdoc
) -> doc::enumdoc {
    doc
}

fn default_seq_fold_res<T>(
    _fold: fold<T>,
    doc: doc::resdoc
) -> doc::resdoc {
    doc
}

fn default_seq_fold_iface<T>(
    _fold: fold<T>,
    doc: doc::ifacedoc
) -> doc::ifacedoc {
    doc
}

fn default_seq_fold_impl<T>(
    _fold: fold<T>,
    doc: doc::impldoc
) -> doc::impldoc {
    doc
}

fn default_seq_fold_type<T>(
    _fold: fold<T>,
    doc: doc::tydoc
) -> doc::tydoc {
    doc
}

#[test]
fn default_fold_should_produce_same_doc() {
    let source = "mod a { fn b() { } mod c { fn d() { } } }";
    let ast = parse::from_str(source);
    let doc = extract::extract(ast, "");
    let fld = default_seq_fold(());
    let folded = fld.fold_crate(fld, doc);
    assert doc == folded;
}

#[test]
fn default_fold_should_produce_same_consts() {
    let source = "const a: int = 0;";
    let ast = parse::from_str(source);
    let doc = extract::extract(ast, "");
    let fld = default_seq_fold(());
    let folded = fld.fold_crate(fld, doc);
    assert doc == folded;
}

#[test]
fn default_fold_should_produce_same_enums() {
    let source = "enum a { b }";
    let ast = parse::from_str(source);
    let doc = extract::extract(ast, "");
    let fld = default_seq_fold(());
    let folded = fld.fold_crate(fld, doc);
    assert doc == folded;
}