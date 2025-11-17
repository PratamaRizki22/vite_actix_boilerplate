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