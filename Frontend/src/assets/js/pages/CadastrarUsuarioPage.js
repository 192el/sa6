import { InputValidation, TYPEVALIDATION } from "../utils/InputValidation.js"


const inputNome = new InputValidation("nome","nomeError", "nomeSucess",TYPEVALIDATION.NAME);
const inputCPF = new InputValidation("CPF","CPFError", "CPFSucess", TYPEVALIDATION.CPF);
inputCPF.onChangeInput();
inputNome.onChangeInput();
