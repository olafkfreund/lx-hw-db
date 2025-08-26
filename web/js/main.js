
// Main JavaScript for Linux Hardware Compatibility Database
document.addEventListener('DOMContentLoaded', () => {
    console.log('Linux Hardware Compatibility Database loaded');
    
    // Add any global functionality here
    initializeSearch();
});

function initializeSearch() {
    const searchInput = document.querySelector('#search-input');
    if (searchInput) {
        searchInput.addEventListener('input', (e) => {
            // Basic search placeholder
            console.log('Search query:', e.target.value);
        });
    }
}
