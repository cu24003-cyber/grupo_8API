use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::NaiveDate;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, PartialEq)]
pub struct AsistenciaClase {
    pub id_asistencia: i32,
    pub id_miembro: i32,        
    pub id_clase: i32,         
    pub fecha_asistencia: NaiveDate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NuevaAsistenciaClase {
    pub id_miembro: i32,        
    pub id_clase: i32,          
    pub fecha_asistencia: NaiveDate,
}

impl NuevaAsistenciaClase {
    /// Valida que los datos sean correctos antes de insertar
    pub fn validar(&self) -> Result<(), String> {
        if self.id_miembro <= 0 {
            return Err("id_miembro debe ser válido".to_string());
        }
        if self.id_clase <= 0 {
            return Err("id_clase debe ser válido".to_string());
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActualizarAsistenciaClase {
    pub id_miembro: i32,        
    pub id_clase: i32,          
    pub fecha_asistencia: NaiveDate,
}

impl ActualizarAsistenciaClase {
    /// Valida que los datos sean correctos antes de actualizar
    pub fn validar(&self) -> Result<(), String> {
        if self.id_miembro <= 0 {
            return Err("id_miembro debe ser válido".to_string());
        }
        if self.id_clase <= 0 {
            return Err("id_clase debe ser válido".to_string());
        }
        Ok(())
    }
}