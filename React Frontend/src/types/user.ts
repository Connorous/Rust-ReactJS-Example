export interface User {
    id: number;
    username: string;
    email: string;
    name: string;
    bio_info: string | null;
    user_type_id: number;
    account_status_id: number;
    status_id: number | null;
    is_online: boolean;
    show_name_choice_id: number;
    theme_id: number;
    theme_dark_mode: boolean;

    // Light mode colours
    light_theme_primary_colour: string;
    light_theme_secondary_colour: string;
    light_theme_accent_colour: string;
    light_theme_sent_colour: string;
    light_theme_received_colour: string;
    light_theme_dark_text_colour: string;
    light_theme_light_text_colour: string;

    // Dark mode colours
    dark_theme_primary_colour: string;
    dark_theme_secondary_colour: string;
    dark_theme_accent_colour: string;
    dark_theme_sent_colour: string;
    dark_theme_received_colour: string;
    dark_theme_dark_text_colour: string;
    dark_theme_light_text_colour: string;
}

export interface Theme {
    id: number;
    theme: string;
}

export interface UserType {
    id: number;
    type: string;
}

export interface UserStatus {
    id: number;
    status: string;
}

export interface ShowNameChoice {
    id: number;
    choice: string;
}

export interface AccountStatus {
    id: number;
    status: string;
}