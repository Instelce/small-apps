

export type CompteType = {
    num: number;
    nom: string;
}

export type ClasseType = {
    name: string;
    description: string;
    comptes: CompteType[];
}

export type LastGameType = {
    score: number;
    errors: number;
}