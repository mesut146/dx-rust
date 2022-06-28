use std::{fs, io};

mod helper;
mod com;

use crate::com::android::dx::cf::direct::DirectClassFile;
use crate::com::android::dx::cf::direct::StdAttributeFactory;
use crate::com::android::dx::cf::iface::ParseException;
use crate::com::android::dx::command::dexer::DxContext;
use crate::com::android::dx::dex::cf::CfOptions;
use crate::com::android::dx::dex::cf::CfTranslator;
use crate::com::android::dx::dex::DexOptions;
use crate::com::android::dx::dex::DexOptions::*;
use crate::com::android::dx::dex::file::DexFile;

struct Tool {}

impl Tool {
    pub fn dexClass(name: &str, bytes: &Vec<u8>) -> Vec<i8> {
        let dexOptions: DexOptions = DexOptions::new();
        let dexFile: DexFile = DexFile::new(&dexOptions);
        let cf: DirectClassFile = Tool::parseClass(name, &bytes);
        let classDefItem: ClassDefItem = Tool::translateClass(&bytes, &cf, &dexFile);
        dexFile.add(&classDefItem);
        return dexFile.toDex(None, false);
    }
    pub fn parseClass(name: &str, bytes: &Vec<u8>) -> DirectClassFile {
        let strictNameCheck: bool = false;
        let cf: DirectClassFile = DirectClassFile::new(&bytes, &name, strictNameCheck);
        cf.setAttributeFactory(&StdAttributeFactory::THE_ONE);
        cf.getMagic();
        return cf;
    }
    pub fn translateClass(bytes: &Vec<u8>, cf: &DirectClassFile, dexFile: &DexFile) -> ClassDefItem {
        let context: DxContext = DxContext::new();
        let cfOptions: CfOptions = CfOptions::new();
        // try        {
        //     return CfTranslator::translate(&context, &cf, &bytes, &cfOptions, dexFile.getDexOptions(), &dexFile);
        // }        catch(        let ex: ParseException)        {
        //     .println_String("\ntrouble processing:");
        //     ex.printContext_PrintStream(&);
        // }
        return None;
    }
}

pub fn main(args: &Vec<String>) {
    let path = "/home/mesut/Desktop/j2cpp-dev/dx-cpp/test/Util.class";
    let data = fs::read(path).unwrap();
    let dex: Vec<i8> = Tool::dexClass("Util.class", &data);
}