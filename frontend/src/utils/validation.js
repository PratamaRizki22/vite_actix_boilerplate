// Untuk validasi form User dan Post
export const validateEmail = (email) => {
  const regex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
  return regex.test(email);
};

export const validateRequired = (value) => {
  return value && value.trim().length > 0;
};

export const validateMinLength = (value, min) => {
  return value.length >= min;
};

// Khusus untuk aplikasi wallet nanti
export const validateWalletAddress = (address) => {
  return address && address.length === 42 && address.startsWith('0x');
};

// Custom validation message styling - Black & White
export const setupCustomValidation = () => {
  const handleInvalid = (e) => {
    e.preventDefault()
    
    const input = e.target
    const message = input.validationMessage || 'Please fill out this field'
    
    // Remove existing error message if any
    const existingError = input.parentElement?.querySelector('.custom-error-message')
    if (existingError) {
      existingError.remove()
    }
    
    // Create custom error message
    const errorDiv = document.createElement('div')
    errorDiv.className = 'custom-error-message'
    errorDiv.style.cssText = `
      color: #000;
      background-color: #fff;
      border: 2px solid #000;
      padding: 0.5rem;
      margin-top: 0.25rem;
      font-weight: bold;
      font-size: 0.875rem;
      display: block;
    `
    errorDiv.textContent = message
    
    // Insert error message after input
    input.parentElement?.insertBefore(errorDiv, input.nextSibling)
    
    // Add border style to input
    input.style.borderColor = '#000'
    input.style.borderWidth = '2px'
    
    // Remove error on input change
    const removeError = () => {
      const err = input.parentElement?.querySelector('.custom-error-message')
      if (err) err.remove()
      input.style.borderColor = ''
      input.style.borderWidth = ''
      input.removeEventListener('input', removeError)
    }
    
    input.addEventListener('input', removeError)
  }
  
  // Find all form inputs and attach invalid handler
  const inputs = document.querySelectorAll('input[required], textarea[required], select[required]')
  inputs.forEach((input) => {
    input.addEventListener('invalid', handleInvalid)
  })
}

// Reinitialize validation for dynamically added forms
export const reinitializeValidation = () => {
  const inputs = document.querySelectorAll('input[required], textarea[required], select[required]')
  inputs.forEach((input) => {
    input.addEventListener('invalid', (e) => {
      e.preventDefault()
      const message = input.validationMessage || 'Please fill out this field'
      
      const existingError = input.parentElement?.querySelector('.custom-error-message')
      if (existingError) {
        existingError.remove()
      }
      
      const errorDiv = document.createElement('div')
      errorDiv.className = 'custom-error-message'
      errorDiv.style.cssText = `
        color: #000;
        background-color: #fff;
        border: 2px solid #000;
        padding: 0.5rem;
        margin-top: 0.25rem;
        font-weight: bold;
        font-size: 0.875rem;
        display: block;
      `
      errorDiv.textContent = message
      
      input.parentElement?.insertBefore(errorDiv, input.nextSibling)
      input.style.borderColor = '#000'
      input.style.borderWidth = '2px'
      
      const removeError = () => {
        const err = input.parentElement?.querySelector('.custom-error-message')
        if (err) err.remove()
        input.style.borderColor = ''
        input.style.borderWidth = ''
        input.removeEventListener('input', removeError)
      }
      
      input.addEventListener('input', removeError)
    })
  })
}
