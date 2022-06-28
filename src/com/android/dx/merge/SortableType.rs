use crate::helper;
use crate::com::android::dx::merge::SortableType;
use crate::com::android::dex::ClassDef;
use crate::com::android::dex::Dex;
use crate::com::android::dex::DexException;
use crate::com::android::dx::merge::IndexMap;

let static NULLS_LAST_ORDER: Comparator<SortableType> = /*new Comparator<SortableType>(){
  @Override public int compare(  SortableType a,  SortableType b){
    if (a == b) {
      return 0;
    }
    if (b == null) {
      return -1;
    }
    if (a == null) {
      return 1;
    }
    if (a.depth != b.depth) {
      return a.depth - b.depth;
    }
    return a.getTypeIndex() - b.getTypeIndex();
  }
}
*/;
struct SortableType{
    pub dex: Dex,
    pub indexMap: IndexMap,
    pub classDef: ClassDef,
    pub depth: i32,
}
impl SortableType{
    pub fn new(&self, dex: &Dex, indexMap: &IndexMap, classDef: &ClassDef)    {
        self->dex=dex;
        self->indexMap=indexMap;
        self->classDef=classDef;
    }
    pub fn getDex(&self) -> Dex    {
        return self.dex;
    }
    pub fn getIndexMap(&self) -> IndexMap    {
        return self.indexMap;
    }
    pub fn getClassDef(&self) -> ClassDef    {
        return self.classDef;
    }
    pub fn getTypeIndex(&self) -> i32    {
        return self.classDef.getTypeIndex();
    }
    pub fn tryAssignDepth(&self, types: &Vec<SortableType>) -> boolean    {
        let max: i32;
        if self.classDef.getSupertypeIndex()==ClassDef::NO_INDEX        {
            max=0;
        }        else         if self.classDef.getSupertypeIndex()==self.classDef.getTypeIndex()        {
            throw DexException::new("Class with type index "+self.classDef.getTypeIndex()+" extends itself");
        }        else         {
            let sortableSupertype: SortableType = types[self.classDef.getSupertypeIndex()];
            if sortableSupertype==None            {
                max=1;
            }            else             if ==-1            {
                return false;
            }            else             {
                max=;
            }
        }
        for interfaceIndex in self.classDef.getInterfaces()        {
            let implemented: SortableType = types[interfaceIndex];
            if implemented==None            {
                max=Math::max_int_int(max, 1);
            }            else             if ==-1            {
                return false;
            }            else             {
                max=Math::max_int_int(max, );
            }
        }
        self.depth=max+1;
        return true;
    }
    pub fn isDepthAssigned(&self) -> boolean    {
        return self.depth!=-1;
    }
}
