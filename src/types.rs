
use crate::evaluator::ObjetoKura;

#[derive(Debug, Clone, PartialEq)]
pub enum TipoKura {
    Entero,
    Cadena,
    Booleano,
    Arreglo(Box<TipoKura>),
    Diccionario(Box<TipoKura>),
    Funcion { params: Vec<TipoKura>, retorno: Box<TipoKura> },
    Desconocido,
}

impl TipoKura {
    pub fn from_string(s: &str) -> Option<Self> {
        match s {
            "Entero" | "int" => Some(TipoKura::Entero),
            "Cadena" | "str" => Some(TipoKura::Cadena),
            "Booleano" | "bool" => Some(TipoKura::Booleano),
            "Arreglo" => Some(TipoKura::Arreglo(Box::new(TipoKura::Desconocido))),
            "Diccionario" => Some(TipoKura::Diccionario(Box::new(TipoKura::Desconocido))),
            _ => None,
        }
    }

    pub fn de_objeto(obj: &ObjetoKura) -> Self {
        match obj {
            ObjetoKura::Entero(_) => TipoKura::Entero,
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
            _ => TipoKura::Desconocido,
        }
    }
    #[allow(dead_code)]
    pub fn to_string(&self) -> String {
        match self {
            TipoKura::Entero => "Entero".to_string(),
            TipoKura::Cadena => "Cadena".to_string(),
            TipoKura::Booleano => "Booleano".to_string(),
            TipoKura::Arreglo(t) => format!("Arreglo<{}>", t.to_string()),
            TipoKura::Diccionario(t) => format!("Diccionario<{}>", t.to_string()),
            TipoKura::Funcion { params, retorno } => {
                let params_str = params.iter().map(|p| p.to_string()).collect::<Vec<_>>().join(", ");
                format!("Funcion<({}) -> {}>", params_str, retorno.to_string())
            },
            TipoKura::Desconocido => "Desconocido".to_string(),
        }
    }
}