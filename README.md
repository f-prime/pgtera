# PGTera

PGTera is a PostgreSQL extension that provides functions for using [Tera](https://keats.github.io/tera/) to render HTML templates. 

When used with a tool like [Postgrest](https://postgrest.org/en/stable/), you can serve HTML directly from the database without managing an intermediate layer.

## Usage

Template files in: `/var/www`
```
/var/www/
  index.html
  about.html
```

Code in `index.html`

```html

<html>
    <title>Home Page</title>
    <body>
        {% for link in links %}
            <a href="{{ link.href | safe }}">{{ link.name }}</a>
        {% endfor %}
    </body>
</html>

```

Render the template in your SQL code. The Second argument of `pgtera_render` is the context which must be passed in as an array of JSON objects that conforms to the following type:

```rust
struct CtxInput {
    name: String,
    value: Value
}
```

Where `Value` is the [serde_json Value enum](https://docs.rs/serde_json/latest/serde_json/value/enum.Value.html). 

```sql
create extension pgtera;

select pgtera_set_render_path('/var/www/**/*.html');

select pgtera_render(
    'index.html',
    $ctx$
    [{
        "name": "links",
        "value": [
            { "name": "Tera Docs", "href": "https://keats.github.io/tera/docs/" },
            { "name": "PGTera", "href": "https://github.com/f-prime/pgtera" }
        ]
    }]
    $ctx$
);
```

Which will return

```html
 <html>                                                                
     <title>Home Page</title>                                          
     <body>                                                            
                                                                       
             <a href="https://keats.github.io/tera/docs/">Tera Docs</a>
                                                                       
             <a href="https://github.com/f-prime/pgtera">PGTera</a>    
                                                                       
     </body>                                                           
 </html>                                                               
```

## Installation

This will install pgtera to the default Postgres installation on your system. For more detailed installation options, checkout the [cargo-pgrx README](https://github.com/pgcentralfoundation/pgrx/blob/develop/cargo-pgrx/README.md)

1. `cargo install cargo-pgrx`
2. `cargo pgrx install`


