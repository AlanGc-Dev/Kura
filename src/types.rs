
use crate::evaluator::ObjetoKura;

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code, unused)]
pub enum TipoKura {
    Entero,
    Flotante(String),  // "f32" o "f64"
    Cadena,
    Booleano,
    Arreglo(Box<TipoKura>),
    Diccionario(Box<TipoKura>),
    Funcion { params: Vec<TipoKura>, retorno: Box<TipoKura> },
    Puntero(Box<TipoKura>),      // 🚀 NUEVO: *T (pointer type)
    Struct(String),              // 🚀 NUEVO: struct name
    Null,                         // 🚀 NUEVO: null type
    Desconocido,
}

impl TipoKura {
    pub fn from_string(s: &str) -> Option<Self> {
        match s {
            "Entero" | "int" => Some(TipoKura::Entero),
            "Flotante" | "f32" => Some(TipoKura::Flotante("f32".to_string())),
            "Flotante64" | "f64" => Some(TipoKura::Flotante("f64".to_string())),
            "Cadena" | "str" => Some(TipoKura::Cadena),
            "Booleano" | "bool" => Some(TipoKura::Booleano),
            "Arreglo" => Some(TipoKura::Arreglo(Box::new(TipoKura::Desconocido))),
            "Diccionario" => Some(TipoKura::Diccionario(Box::new(TipoKura::Desconocido))),
            "null" => Some(TipoKura::Null),  // 🚀 NUEVO
            s if s.starts_with("*") => {  // 🚀 NUEVO: Pointer syntax
                let inner = Self::from_string(&s[1..])?;
                Some(TipoKura::Puntero(Box::new(inner)))
            },
            _ => None,
        }
    }

    pub fn de_objeto(obj: &ObjetoKura) -> Self {
        match obj {
            ObjetoKura::Entero(_) => TipoKura::Entero,
            ObjetoKura::Flotante(_) => TipoKura::Flotante("f64".to_string()),  // 🚀 NUEVO
            ObjetoKura::Cadena(_) => TipoKura::Cadena,
            ObjetoKura::Booleano(_) => TipoKura::Booleano,
            ObjetoKura::Arreglo(arr) => {
                if arr.is_empty() {
                    TipoKura::Arreglo(Box::new(TipoKura::Desconocido))
                } else {
                    TipoKura::Arreglo(Box::new(TipoKura::de_objeto(&arr[0])))
                }
            },

            ObjetoKura::Diccionario(_) => TipoKura::Diccionario(Box::new(TipoKura::Desconocido)),
            ObjetoKura::Funcion { .. } => TipoKura::Funcion {
                params: vec![],
                retorno: Box::new(TipoKura::Desconocido),
            },
            ObjetoKura::Puntero(rc_ref) => {  // 🚀 NUEVO
                let obj = rc_ref.borrow();
                TipoKura::Puntero(Box::new(TipoKura::de_objeto(&obj)))
            },
            ObjetoKura::InstanciaStruct { nombre, .. } => {  // 🚀 NUEVO
                TipoKura::Struct(nombre.clone())
            },
            _ => TipoKura::Desconocido,
        }
    }
    #[allow(dead_code)]
    pub fn to_string(&self) -> String {
        match self {
            TipoKura::Entero => "Entero".to_string(),
            TipoKura::Flotante(precision) => format!("Flotante({})", precision),
            TipoKura::Cadena => "Cadena".to_string(),
            TipoKura::Booleano => "Booleano".to_string(),
            TipoKura::Arreglo(t) => format!("Arreglo<{}>", t.to_string()),
            TipoKura::Diccionario(t) => format!("Diccionario<{}>", t.to_string()),
            TipoKura::Funcion { params, retorno } => {
                let params_str = params.iter().map(|p| p.to_string()).collect::<Vec<_>>().join(", ");
                format!("Funcion<({}) -> {}>", params_str, retorno.to_string())
            },
            TipoKura::Puntero(t) => format!("*{}", t.to_string()),  // 🚀 NUEVO
            TipoKura::Struct(nombre) => format!("struct {}", nombre),  // 🚀 NUEVO
            TipoKura::Null => "null".to_string(),  // 🚀 NUEVO
            TipoKura::Desconocido => "Desconocido".to_string(),
        }
    }
}