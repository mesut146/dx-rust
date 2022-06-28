use crate::helper;
use crate::com::android::dx::util::ByteArray;
use crate::com::android::dx::util::TwoColumnOutput;
use crate::com::android::dx::util::Hex;
use crate::com::android::dex::Leb128;
use crate::com::android::dx::util::ByteArrayAnnotatedOutput::Annotation;
use crate::com::android::dex::util::ExceptionWithContext;

struct ByteArrayAnnotatedOutput{
    pub stretchy: boolean,
    pub data: Vec<i8>,
    pub cursor: i32,
    pub verbose: boolean,
    pub annotations: ArrayList<Annotation>,
    pub annotationWidth: i32,
    pub hexCols: i32,
}
impl ByteArrayAnnotatedOutput{
    pub const DEFAULT_SIZE: i32 = 1000;
    pub fn new(&self, data: &Vec<i8>)    {
        this(data,false);

    }
    pub fn new(&self)    {
        this(DEFAULT_SIZE);

    }
    pub fn new(&self, size: i32)    {
        this(new byte[size],true);

    }
    pub fn new(&self, data: &Vec<i8>, stretchy: boolean)    {
        if data==None        {
            throw NullPointerException::new("data == null");
        }        
        self->stretchy=stretchy;
        self->data=data;
        self->cursor=0;
        self->verbose=false;
        self->annotations=None;
        self->annotationWidth=0;
        self->hexCols=0;
    }
    pub fn getArray(&self) -> Vec<i8>    {
        return self.data;
    }
    pub fn toByteArray(&self) -> Vec<i8>    {
        let result: Vec<i8> = new byte[cursor];
        System::arraycopy(&self.data, 0, &result, 0, self.cursor);
        return result;
    }
    pub fn getCursor(&self) -> i32    {
        return self.cursor;
    }
    pub fn assertCursor(&self, expectedCursor: i32)    {
        if self.cursor!=expectedCursor        {
            throw ExceptionWithContext::new("expected cursor "+expectedCursor+"; actual value: "+self.cursor);
        }        
    }
    pub fn writeByte(&self, value: i32)    {
        let writeAt: i32 = self.cursor;
        let end: i32 = writeAt+1;
        if self.stretchy        {
            ensureCapacity(end);
        }        else         if end>self.data.len()        {
            ByteArrayAnnotatedOutput::throwBounds();
            return;
        }        
        self.data[writeAt]=value as i8;
        self.cursor=end;
    }
    pub fn writeShort(&self, value: i32)    {
        let writeAt: i32 = self.cursor;
        let end: i32 = writeAt+2;
        if self.stretchy        {
            ensureCapacity(end);
        }        else         if end>self.data.len()        {
            ByteArrayAnnotatedOutput::throwBounds();
            return;
        }        
        self.data[writeAt]=value as i8;
        self.data[writeAt+1]=(value>>8) as i8;
        self.cursor=end;
    }
    pub fn writeInt(&self, value: i32)    {
        let writeAt: i32 = self.cursor;
        let end: i32 = writeAt+4;
        if self.stretchy        {
            ensureCapacity(end);
        }        else         if end>self.data.len()        {
            ByteArrayAnnotatedOutput::throwBounds();
            return;
        }        
        self.data[writeAt]=value as i8;
        self.data[writeAt+1]=(value>>8) as i8;
        self.data[writeAt+2]=(value>>16) as i8;
        self.data[writeAt+3]=(value>>24) as i8;
        self.cursor=end;
    }
    pub fn writeLong(&self, value: i64)    {
        let writeAt: i32 = self.cursor;
        let end: i32 = writeAt+8;
        if self.stretchy        {
            ensureCapacity(end);
        }        else         if end>self.data.len()        {
            ByteArrayAnnotatedOutput::throwBounds();
            return;
        }        
        let half: i32 = value as i32;
        self.data[writeAt]=half as i8;
        self.data[writeAt+1]=(half>>8) as i8;
        self.data[writeAt+2]=(half>>16) as i8;
        self.data[writeAt+3]=(half>>24) as i8;
        half=(value>>32) as i32;
        self.data[writeAt+4]=half as i8;
        self.data[writeAt+5]=(half>>8) as i8;
        self.data[writeAt+6]=(half>>16) as i8;
        self.data[writeAt+7]=(half>>24) as i8;
        self.cursor=end;
    }
    pub fn writeUleb128(&self, value: i32) -> i32    {
        if self.stretchy        {
            ensureCapacity(self.cursor+5);
        }        
        let cursorBefore: i32 = self.cursor;
        Leb128::writeUnsignedLeb128(self, value);
        return (self.cursor-cursorBefore);
    }
    pub fn writeSleb128(&self, value: i32) -> i32    {
        if self.stretchy        {
            ensureCapacity(self.cursor+5);
        }        
        let cursorBefore: i32 = self.cursor;
        Leb128::writeSignedLeb128(self, value);
        return (self.cursor-cursorBefore);
    }
    pub fn write(&self, bytes: &ByteArray)    {
        let blen: i32 = bytes.size();
        let writeAt: i32 = self.cursor;
        let end: i32 = writeAt+blen;
        if self.stretchy        {
            ensureCapacity(end);
        }        else         if end>self.data.len()        {
            ByteArrayAnnotatedOutput::throwBounds();
            return;
        }        
        bytes.getBytes(&self.data, writeAt);
        self.cursor=end;
    }
    pub fn write(&self, bytes: &Vec<i8>, offset: i32, length: i32)    {
        let writeAt: i32 = self.cursor;
        let end: i32 = writeAt+length;
        let bytesEnd: i32 = offset+length;
        if ((offset|length|end)<0)||(bytesEnd>bytes.len())        {
            throw IndexOutOfBoundsException::new("bytes.length "+bytes.len()+"; "+offset+"..!"+end);
        }        
        if self.stretchy        {
            ensureCapacity(end);
        }        else         if end>self.data.len()        {
            ByteArrayAnnotatedOutput::throwBounds();
            return;
        }        
        System::arraycopy(&bytes, offset, &self.data, writeAt, length);
        self.cursor=end;
    }
    pub fn write(&self, bytes: &Vec<i8>)    {
        write_byte[]_int_int(&bytes, 0, bytes.len());
    }
    pub fn writeZeroes(&self, count: i32)    {
        if count<0        {
            throw IllegalArgumentException::new("count < 0");
        }        
        let end: i32 = self.cursor+count;
        if self.stretchy        {
            ensureCapacity(end);
        }        else         if end>self.data.len()        {
            ByteArrayAnnotatedOutput::throwBounds();
            return;
        }        
        Arrays::fill_byte[]_int_int_byte(&self.data, self.cursor, end, 0 as i8);
        self.cursor=end;
    }
    pub fn alignTo(&self, alignment: i32)    {
        let mask: i32 = alignment-1;
        if (alignment<0)||((mask&alignment)!=0)        {
            throw IllegalArgumentException::new("bogus alignment");
        }        
        let end: i32 = (self.cursor+mask)&~mask;
        if self.stretchy        {
            ensureCapacity(end);
        }        else         if end>self.data.len()        {
            ByteArrayAnnotatedOutput::throwBounds();
            return;
        }        
        Arrays::fill_byte[]_int_int_byte(&self.data, self.cursor, end, 0 as i8);
        self.cursor=end;
    }
    pub fn annotates(&self) -> boolean    {
        return (self.annotations!=None);
    }
    pub fn isVerbose(&self) -> boolean    {
        return self.verbose;
    }
    pub fn annotate(&self, msg: &String)    {
        if self.annotations==None        {
            return;
        }        
        endAnnotation();
        self.annotations.add_Annotation(Annotation::new(self.cursor, &msg));
    }
    pub fn annotate(&self, amt: i32, msg: &String)    {
        if self.annotations==None        {
            return;
        }        
        endAnnotation();
        let asz: i32 = self.annotations.size();
        let lastEnd: i32 = if (asz==0) { 0 } else { self.annotations.get(asz-1).getEnd() };
                let startAt: i32;
                if lastEnd<=self.cursor                {
                    startAt=self.cursor;
                }                else                 {
                    startAt=lastEnd;
                }
                self.annotations.add_Annotation(Annotation::new(startAt, startAt+amt, &msg));
            }
            pub fn endAnnotation(&self)            {
                if self.annotations==None                {
                    return;
                }                
                let sz: i32 = self.annotations.size();
                if sz!=0                {
                    self.annotations.get(sz-1).setEndIfUnset(self.cursor);
                }                
            }
            pub fn getAnnotationWidth(&self) -> i32            {
                let leftWidth: i32 = 8+(self.hexCols*2)+(self.hexCols/2);
                return self.annotationWidth-leftWidth;
            }
            pub fn enableAnnotations(&self, annotationWidth: i32, verbose: boolean)            {
                if (self.annotations!=None)||(self.cursor!=0)                {
                    throw RuntimeException::new("cannot enable annotations");
                }                
                if annotationWidth<40                {
                    throw IllegalArgumentException::new("annotationWidth < 40");
                }                
                let hexCols: i32 = (((annotationWidth-7)/15)+1)&~1;
                if hexCols<6                {
                    hexCols=6;
                }                else                 if hexCols>10                {
                    hexCols=10;
                }                
                self->annotations=ArrayList<Annotation>::new(1000);
                self->annotationWidth=annotationWidth;
                self->hexCols=hexCols;
                self->verbose=verbose;
            }
            pub fn finishAnnotating(&self)            {
                endAnnotation();
                if self.annotations!=None                {
                    let asz: i32 = self.annotations.size();
                    while asz>0                    {
                        let last: Annotation = self.annotations.get(asz-1);
                        if last.getStart()>self.cursor                        {
                            self.annotations.remove_int(asz-1);
                            asz -= 1;
                        }                        else                         if last.getEnd()>self.cursor                        {
                            last.setEnd(self.cursor);
                            break;
                        }                        else                         {
                            break;
                        }
                    }
                }                
            }
            pub fn writeAnnotationsTo(&self, out: &Writer)            {
                let width2: i32 = getAnnotationWidth();
                let width1: i32 = self.annotationWidth-width2-1;
                let twoc: TwoColumnOutput = TwoColumnOutput::new(&out, width1, width2, "|");
                let left: Writer = twoc.getLeft();
                let right: Writer = twoc.getRight();
                let leftAt: i32 = 0;
                let rightAt: i32 = 0;
                let rightSz: i32 = self.annotations.size();
                while (leftAt<self.cursor)&&(rightAt<rightSz)                {
                    let a: Annotation = self.annotations.get(rightAt);
                    let start: i32 = a.getStart();
                    let end: i32;
                    let text: String;
                    if leftAt<start                    {
                        end=start;
                        start=leftAt;
                        text="";
                    }                    else                     {
                        end=a.getEnd();
                        text=a.getText();
                        rightAt += 1;
                    }
                    left.write_String(Hex::dump(&self.data, start, end-start, start, self.hexCols, 6));
                    right.write_String(&text);
                    twoc.flush();
                    leftAt=end;
                }
                if leftAt<self.cursor                {
                    left.write_String(Hex::dump(&self.data, leftAt, self.cursor-leftAt, leftAt, self.hexCols, 6));
                }                
                while rightAt<rightSz                {
                    right.write_String(self.annotations.get(rightAt).getText());
                    rightAt += 1;
                }
                twoc.flush();
            }
            pub fn throwBounds()            {
                throw IndexOutOfBoundsException::new("attempt to write past the end");
            }
            pub fn ensureCapacity(&self, desiredSize: i32)            {
                if self.data.len()<desiredSize                {
                    let newData: Vec<i8> = new byte[desiredSize * 2 + 1000];
                    System::arraycopy(&self.data, 0, &newData, 0, self.cursor);
                    self.data=newData;
                }                
            }
}
