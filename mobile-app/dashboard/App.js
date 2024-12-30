import React from 'react';
import { SafeAreaView } from 'react-native';
import Dashboard from './components/Dashboard';

const App = () => {
    return (
        <SafeAreaView style={{ flex: 1 }}>
            <Dashboard />
        </SafeAreaView>
    );
};

export default App;
