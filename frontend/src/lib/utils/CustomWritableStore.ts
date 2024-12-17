interface CustomWritableStore<SubT, SetT> {
    unsubscribe(): void;
    subscribe(subscription: (value: SubT) => void): () => void;
    set?(value: SetT): void;
}
