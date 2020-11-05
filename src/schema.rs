table! {
    keys (user, name) {
        user -> Text,
        name -> Text,
        public_key -> Binary,
        private_key -> Binary,
    }
}
