import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Enerledger } from "../target/types/enerledger";

describe("enerledger", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.Enerledger as Program<Enerledger>;

  it("Is initialized!", async () => {
    console.log("¡El programa Enerledger se ha cargado correctamente!");
  });
});
