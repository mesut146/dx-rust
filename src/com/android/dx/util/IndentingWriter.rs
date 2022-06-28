use crate::helper;

struct IndentingWriter{
    pub prefix: String,
    pub width: i32,
    pub maxIndent: i32,
    pub column: i32,
    pub collectingIndent: boolean,
    pub indent: i32,
}
impl IndentingWriter{
    pub fn new(&self, out: &Writer, width: i32, prefix: &String)    {
        super(out);

        if out==None        {
            throw NullPointerException::new("out == null");
        }        
        if width<0        {
            throw IllegalArgumentException::new("width < 0");
        }        
        if prefix==None        {
            throw NullPointerException::new("prefix == null");
        }        
        self->width=if (width!=0) { width } else { Integer::MAX_VALUE };
                self->maxIndent=width>>1;
                self->prefix=if (prefix.length()==0) { None } else { prefix };
                        bol();
                    }
                    pub fn new(&self, out: &Writer, width: i32)                    {
                        this(out,width,"");

                    }
                    pub fn write(&self, c: i32)                    {
//synchronized (lock) {
  if (collectingIndent) {
    if (c == ' ') {
      indent++;
      if (indent >= maxIndent) {
        indent=maxIndent;
        collectingIndent=false;
      }
    }
 else {
      collectingIndent=false;
    }
  }
  if ((column == width) && (c != '\n')) {
    out.write('\n');
    column=0;
  }
  if (column == 0) {
    if (prefix != null) {
      out.write(prefix);
    }
    if (!collectingIndent) {
      for (int i=0; i < indent; i++) {
        out.write(' ');
      }
      column=indent;
    }
  }
  out.write(c);
  if (c == '\n') {
    bol();
  }
 else {
    column++;
  }
}

                    }
                    pub fn write(&self, cbuf: &Vec<char>, off: i32, len: i32)                    {
//synchronized (lock) {
  while (len > 0) {
    write(cbuf[off]);
    off++;
    len--;
  }
}

                    }
                    pub fn write(&self, str: &String, off: i32, len: i32)                    {
//synchronized (lock) {
  while (len > 0) {
    write(str.charAt(off));
    off++;
    len--;
  }
}

                    }
                    pub fn bol(&self)                    {
                        self.column=0;
                        self.collectingIndent=(self.maxIndent!=0);
                        self.indent=0;
                    }
}
