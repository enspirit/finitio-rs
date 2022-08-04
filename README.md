# Finitio

Finitio is a language for capturing information structure. A little bit like
"JSON/XML schema" but on steroids. An example is shown below. For more
information about Finitio itself, see [www.finitio.io](http://www.finitio.io)

```finitio
@import finitio/data

Uuid = String( s | s =~ /^[a-z0-9-]{36}$/ )
Name = String( s | s.length > 0 )
Temp = <celsius> Real( f | f >= 33.0 && f <= 45.0 )
{
  patient : {
    id   : Uuid
    name : Name
    dob  : Date( d | alive: d.year > 1890 )
  }
  symptoms : [ String( s | s.size > 0 ) ]
  temperature : Temp
}
```

Finitio-rs is a parser and validator written in rust.

