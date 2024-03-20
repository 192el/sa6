use super::TRepository::TRepository;
use crate::connection::estabilishConnection;
use crate::entities::Academia::{Academia, NovaAcademia};
use crate::schema::academiausuarios::AcademiaId;
use crate::schema::{academia, academia::dsl::*};
use actix_web::web::Data;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::RunQueryDsl;
use diesel::*;
pub struct AcademiaRepository {
    conn: MysqlConnection,
}
impl AcademiaRepository {
    pub fn new() -> AcademiaRepository {
        AcademiaRepository {
            conn: estabilishConnection(),
        }
    }
}
impl TRepository<Academia> for AcademiaRepository {
    async fn salvar(&mut self, entidade: Academia) -> Result<(), Error> {
        let nova_academia = NovaAcademia {
            NomeComercial: entidade.NomeComercial,
            CapacidadeUsuarios: entidade.CapacidadeUsuarios,
            Endereco: entidade.Endereco,
            HorarioAbertura: entidade.HorarioAbertura,
            HorarioFechamento: entidade.HorarioFechamento,
        };
        let result = diesel::insert_into(academia::table)
            .values(&nova_academia)
            .execute(&mut self.conn);
        if result.is_ok() {
            Ok(())
        } else {
            Err(()).expect("Error ao salvar entidade")
        }
    }

    async fn listar(&mut self) -> Result<Vec<Academia>, Error> {
        let academia_table = academia;
        let mut listaAcademias: Vec<Academia> = Vec::new();
        match academia_table.load::<Academia>(&mut self.conn) {
            Ok(results) => {
                for academiaData in results {
                    let academia_entities = Academia::new(
                        academiaData.Id,
                        academiaData.NomeComercial,
                        academiaData.HorarioAbertura,
                        academiaData.HorarioFechamento,
                        academiaData.CapacidadeUsuarios,
                        academiaData.Endereco,
                    );
                    listaAcademias.push(academia_entities)
                }
                return Ok(listaAcademias);
            }
            Err(err) => {
                println!("Ocorreu o seguinte erro: {}!", err);
                return Err(()).expect("Erro ao puxar informações");
            }
        }
    }

    async fn update(&mut self, entidade: Academia) -> Result<(), Error> {
        let result = update(academia)
            .set((
                NomeComercial.eq(entidade.NomeComercial),
                HorarioAbertura.eq(entidade.HorarioAbertura),
                HorarioFechamento.eq(entidade.HorarioFechamento),
                Endereco.eq(entidade.Endereco),
                CapacidadeUsuarios.eq(entidade.CapacidadeUsuarios)
            ))
            .filter(Id.eq(entidade.Id))
            .execute(&mut self.conn);
        if result.is_ok() {
            Ok(())
        } else {
            Err(NotFound)
        }
    }

    async fn delete(&mut self, entidade: Academia) -> Result<(), Error> {
        let result = delete(academia)
            .filter(Id.eq(entidade.Id))
            .execute(&mut self.conn);
        if result.is_ok() {
            Ok(())
        } else {
            Err(()).expect("Error ao deletar entidade")
        }
    }

    async fn findById(&mut self, _id: i32) -> Result<Academia, Error> {
        let result: Result<Academia, Error> = academia::table
            .filter(academia::Id.eq(_id))
            .first::<Academia>(&mut self.conn);
        match result {
            Ok(academia_result) => {
                return Ok(academia_result);
            }
            Err(e) => {
                return Err(()).expect("Erro ao buscar dado");
            }
        }
    }

    async fn deleteById(&mut self, _id: i32) -> Result<(), Error> {
        let result = delete(academia).filter(Id.eq(_id)).execute(&mut self.conn);
        if result.is_ok() {
            Ok(())
        } else {
            Err(()).expect("Error ao deletar entidade")
        }
    }
}
