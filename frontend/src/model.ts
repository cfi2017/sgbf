
export interface Day {
    entries: RosterEntry[];
    action: EditAction;
    id?: number;
    participantType: ParticipantType;
    format: string,
    remarks?: string;
    entryType?: RosterEntryType;
}

export interface User {
    id: string;
    name: string;
    settings: {
        notifications: {
            enabled: boolean;
            flightInstructors: boolean;
            potentialFlightInstructors: boolean;
            flightInstructorRequests: boolean;
            towPilots: boolean;
            potentialTowPilots: boolean;
            towPilotRequests: boolean;
        }
    }
}

export enum EditAction {
    Add = 'add',
    Edit = 'edit',
}

export enum ParticipantType {
    GliderPilot = "participant_sf"
}

export interface RosterEntry {
    name: string;
    message: string;
    entryType: RosterEntryType;
}

export enum RosterEntryType {
    Definite = 'Definite',
    Tentative = 'Tentative',
    Unavailable = 'Unavailable',
}

export interface DayOverview {
    date: string; // equivalent to chrono::NaiveDate in JavaScript
    registeredPilots: Stats;
    entries: PersonEntry[];
    note?: string;
}

export interface Stats {
    definitive: number;
    tentative: number;
}

export interface PersonEntry {
    timeFrame: TimeFrame; // equivalent to (chrono::NaiveTime, chrono::NaiveTime) in JavaScript
    name: string;
    entryType: EntryType;
    note1?: string;
    note2?: string;
}

export type TimeFrame = [string, string]; // NaiveTime in JavaScript could be represented as string

export enum EntryType {
    FlightInstructor = 'FlightInstructor',
    TowingPilot = 'TowingPilot',
    WinchOperator = 'WinchOperator',
}

export interface Period {
    from: string,
    to: string,
}

export interface Reservation {
    id: number,
    plane: string,
    reservedBy: string,
    createdAt: string,
    period: Period,
    comments: string[]
}
