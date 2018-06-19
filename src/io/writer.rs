use curie::PrefixMapping;
use model::*;
use std::io::Write;

use quick_xml::events::BytesEnd;
use quick_xml::events::BytesStart;
use quick_xml::events::BytesDecl;
use quick_xml::events::Event;
use quick_xml::writer::Writer;

pub fn write<W: Write>(write: &mut W, ont: &Ontology,
                       _prefix: Option<&PrefixMapping>){
    let mut writer = Writer::new_with_indent(write, ' ' as u8, 4);

    writer.write_event(Event::Decl(BytesDecl::new(&b"1.0"[..], None, None))).ok();

    let mut elem = BytesStart::owned(b"Ontology".to_vec(),
                                     "Ontology".len());
    elem.push_attribute(("xmlns","http://www.w3.org/2002/07/owl#"));
    push_iri_attribute_maybe(&mut elem, ont,
                             "ontologyIRI",ont.id.iri);
    push_iri_attribute_maybe(&mut elem, ont,
                             "versionIRI",ont.id.viri);

    writer.write_event(Event::Start(elem)).ok();

    let elem = BytesEnd::owned(b"Ontology".to_vec());

    push_classes(&mut writer, ont, _prefix);
    writer.write_event(Event::End(elem)).ok();
}

fn push_iri_attribute_maybe(elem:&mut BytesStart,ont:&Ontology,
                            key:&str,iri:Option<IRI>)
{
    match iri{
        Some(iri) => {
            let iri = ont.iri_to_str(iri).unwrap();
            elem.push_attribute((key, &iri[..]));
        },
        None => {}
    }
}

pub fn push_classes<W: Write>(writer: &mut Writer<W>, ont: &Ontology,
                              _prefix: Option<&PrefixMapping>){

    // Make rendering determinisitic in terms of order
    let mut classes:Vec<&String> = ont.class.iter().
        map(|c|
            ont.iri_to_str(c.0).unwrap()
        ).
        collect::<Vec<&String>>();

    classes.sort();

    for iri in classes {
        let mut declaration = BytesStart::owned(b"Declaration".to_vec(),
                                                "Declaration".len());

        writer.write_event(Event::Start(declaration)).ok();

        let mut class = BytesStart::owned(b"Class".to_vec(),"Class".len());
        class.push_attribute(("IRI",&iri[..]));
        writer.write_event(Event::Start(class)).ok();

        writer.write_event(Event::End(BytesEnd::owned(b"Class".to_vec()))).ok();
        writer.write_event(Event::End(
            BytesEnd::owned(b"Declaration".to_vec()))).ok();
    }
}

#[cfg(test)]
mod test{

    extern crate mktemp;

    use super::*;
    use io::reader::*;
    use self::mktemp::Temp;

    use std::fs::File;
    use std::io::BufReader;
    use std::io::BufWriter;

    #[test]
    fn test_ont_rt(){
        let mut ont = Ontology::new();
        let iri = ont.iri("http://www.example.com/a".to_string());
        ont.id.iri = Some(iri);
        let temp_file = Temp::new_file().unwrap();
        let file = File::create(&temp_file).ok().unwrap();
        write(&mut BufWriter::new(file), &ont, None);

        let file = File::open(&temp_file).ok().unwrap();
        let (ont2,_) = read(&mut BufReader::new(file));

        assert_eq!(ont.iri_to_str(ont.id.iri.unwrap()).unwrap(),
                   ont2.iri_to_str(ont2.id.iri.unwrap()).unwrap());
    }



}