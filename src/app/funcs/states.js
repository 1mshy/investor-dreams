/**
 * This file is used to store the state of the application.
 * This is useful for the following reasons:
 * - The state can be accessed from anywhere in the application
 * - The state can be updated from anywhere in the application
 * - The state can be reset to its default state
 * etc.
 * 
 * 6/11/24 - currently being used to store the state of the analysis fetcher so that it will run even when exiting the analysis page
 */

let state = {}

export function set_state(new_state) {
    state = new_state;
}

export function get_state() {
    return state;
}