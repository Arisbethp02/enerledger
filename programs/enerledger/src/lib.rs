use anchor_lang::prelude::*;

declare_id!("HiTjBaHUp7t6ETtZNEZdfKJ7gCTSAJn5uY8jzgNHsE6j");

#[program]
pub mod enerledger {
    use super::*;

    // CREATE: Inicializa el medidor usando un PDA
    pub fn inicializar_medidor(ctx: Context<InicializarMedidor>, id_fisico: String) -> Result<()> {
        let registro = &mut ctx.accounts.registro_pda;
        registro.id_fisico = id_fisico;
        registro.total_kwh = 0;
        registro.autoridad = ctx.accounts.autoridad.key();
        Ok(())
    }

    // UPDATE: Registra nueva lectura
    pub fn registrar_lectura(ctx: Context<ActualizarMedidor>, nueva_lectura: u64) -> Result<()> {
        let registro = &mut ctx.accounts.registro_pda;
        registro.total_kwh += nueva_lectura;
        Ok(())
    }

    // DELETE: Cierra la cuenta y recupera los SOL (opcional para CRUD completo)
    pub fn eliminar_registro(_ctx: Context<EliminarMedidor>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(id_fisico: String)]
pub struct InicializarMedidor<'info> {
    // Aquí ocurre la magia del PDA: se deriva de la palabra "medidor" y el ID físico
    #[account(
        init, 
        payer = autoridad, 
        space = 8 + 32 + 64 + 8, 
        seeds = [b"medidor", id_fisico.as_bytes()], 
        bump
    )]
    pub registro_pda: Account<'info, RegistroEnergia>,
    #[account(mut)]
    pub autoridad: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ActualizarMedidor<'info> {
    #[account(mut, has_one = autoridad)]
    pub registro_pda: Account<'info, RegistroEnergia>,
    pub autoridad: Signer<'info>,
}

#[derive(Accounts)]
pub struct EliminarMedidor<'info> {
    #[account(mut, has_one = autoridad, close = autoridad)]
    pub registro_pda: Account<'info, RegistroEnergia>,
    pub autoridad: Signer<'info>,
}

#[account]
pub struct RegistroEnergia {
    pub autoridad: Pubkey,
    pub id_fisico: String,
    pub total_kwh: u64,
}