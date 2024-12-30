import { createStore } from 'redux';

// Initial state
const initialState = {
    // Define your initial state here
};

// Reducer
const reducer = (state = initialState, action: any) => {
    switch (action.type) {
        // Handle your actions here
        default:
            return state;
    }
};

// Create store
const store = createStore(reducer);

export default store;
