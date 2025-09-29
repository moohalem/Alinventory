document.addEventListener('DOMContentLoaded', () => {
    const tableBody = document.getElementById('inventory-table-body');
    const formContainer = document.getElementById('form-container');
    const formTitle = document.getElementById('form-title');
    const nameInput = document.getElementById('name');
    const quantityInput = document.getElementById('quantity');
    const unitInput = document.getElementById('unit');
    const originalNameInput = document.getElementById('original-name');

    const addBtn = document.getElementById('add-btn');
    const editBtn = document.getElementById('edit-btn');
    const deleteBtn = document.getElementById('delete-btn');
    const saveBtn = document.getElementById('save-button');
    const cancelBtn = document.getElementById('cancel-button');

    // --- API Functions ---

    const api = {
        async getIngredients() {
            const response = await fetch('/api/ingredients');
            return response.json();
        },
        async addIngredient(ingredient) {
            const response = await fetch('/api/ingredients', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify(ingredient),
            });
            if (!response.ok) {
                console.error('API call failed:', response.status, await response.text());
                throw new Error('Failed to add/update ingredient');
            }
        },
        async deleteIngredients(names) {
            await fetch('/api/ingredients', {
                method: 'DELETE',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ names }),
            });
        },
    };

    // --- UI Functions ---

    const formatTimestamp = (isoString) => {
        const date = new Date(isoString);
        const year = date.getFullYear();
        const month = (date.getMonth() + 1).toString().padStart(2, '0');
        const day = date.getDate().toString().padStart(2, '0');
        const hours = date.getHours().toString().padStart(2, '0');
        const minutes = date.getMinutes().toString().padStart(2, '0');
        return `${year}-${month}-${day} ${hours}:${minutes}`;
    };

    const renderTable = (ingredients) => {
        tableBody.innerHTML = '';
        ingredients.forEach(ingredient => {
            const row = document.createElement('tr');
            row.innerHTML = `
                <td><input type="checkbox" class="ingredient-checkbox" value="${ingredient.name}"></td>
                <td>${ingredient.name}</td>
                <td>${ingredient.quantity}</td>
                <td>${ingredient.unit}</td>
                <td>${formatTimestamp(ingredient.last_edited)}</td>
            `;
            tableBody.appendChild(row);
        });
    };

    const showForm = (edit = false, ingredient = null) => {
        if (edit && ingredient) {
            formTitle.textContent = 'Edit Ingredient';
            nameInput.value = ingredient.name;
            quantityInput.value = ingredient.quantity;
            unitInput.value = ingredient.unit;
            originalNameInput.value = ingredient.name;
            nameInput.readOnly = true; // Name is read-only when editing
            quantityInput.readOnly = false; // Ensure quantity is editable
            unitInput.disabled = false; // Ensure unit is editable
        } else {
            formTitle.textContent = 'Add New Ingredient';
            nameInput.value = '';
            quantityInput.value = '';
            unitInput.value = 'Gram';
            originalNameInput.value = '';
            nameInput.readOnly = false; // Name is editable when adding
            quantityInput.readOnly = false; // Ensure quantity is editable
            unitInput.disabled = false; // Ensure unit is editable
        }
        formContainer.style.display = 'block';
    };

    const hideForm = () => {
        formContainer.style.display = 'none';
    };

    const refreshIngredients = async () => {
        const ingredients = await api.getIngredients();
        renderTable(ingredients);
    };

    // --- Event Listeners ---

    addBtn.addEventListener('click', () => showForm(false));

    editBtn.addEventListener('click', async () => {
        const selectedCheckbox = document.querySelector('.ingredient-checkbox:checked');
        if (!selectedCheckbox) {
            alert('Please select an ingredient to edit.');
            return;
        }
        const ingredients = await api.getIngredients();
        const ingredientToEdit = ingredients.find(i => i.name === selectedCheckbox.value);
        showForm(true, ingredientToEdit);
    });

    deleteBtn.addEventListener('click', async () => {
        const selectedCheckboxes = document.querySelectorAll('.ingredient-checkbox:checked');
        if (selectedCheckboxes.length === 0) {
            alert('Please select ingredients to delete.');
            return;
        }
        const namesToDelete = Array.from(selectedCheckboxes).map(cb => cb.value);
        if (confirm(`Are you sure you want to delete ${namesToDelete.length} ingredient(s)?`)) {
            await api.deleteIngredients(namesToDelete);
            await refreshIngredients();
        }
    });

    saveBtn.addEventListener('click', async () => {
        const ingredient = {
            name: nameInput.value,
            quantity: parseInt(quantityInput.value, 10),
            unit: unitInput.value,
        };

        if (!ingredient.name || isNaN(ingredient.quantity)) {
            alert('Please enter a valid name and quantity.');
            return;
        }
        
        await api.addIngredient(ingredient);
        hideForm();
        await refreshIngredients();
    });

    cancelBtn.addEventListener('click', hideForm);

    // --- Initial Load ---

    refreshIngredients();
});
