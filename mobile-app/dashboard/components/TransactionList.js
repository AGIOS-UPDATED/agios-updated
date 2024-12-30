import React from 'react';
import { View, Text, FlatList, StyleSheet } from 'react-native';

const TransactionList = ({ transactions }) => {
    return (
        <View style={styles.container}>
            <FlatList
                data={transactions}
                keyExtractor={(item) => item.id.toString()}
                renderItem={({ item }) => (
                    <View style={styles.item}>
                        <Text>{item.description}</Text>
                        <Text>${item.amount}</Text>
                    </View>
                )}
            />
        </View>
    );
};

const styles = StyleSheet.create({
    container: {
        padding: 10,
    },
    item: {
        padding: 15,
        borderBottomWidth: 1,
        borderBottomColor: '#ccc',
    },
});

export default TransactionList;
