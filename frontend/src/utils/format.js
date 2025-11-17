// Format data untuk display
export const formatDate = (dateString) => {
  return new Date(dateString).toLocaleDateString('id-ID');
};

export const truncateText = (text, maxLength = 50) => {
  if (text.length <= maxLength) return text;
  return text.substring(0, maxLength) + '...';
};

// Untuk wallet nanti
export const formatCryptoAmount = (amount, decimals = 4) => {
  return parseFloat(amount).toFixed(decimals);
};

export const truncateAddress = (address, start = 6, end = 4) => {
  if (!address) return '';
  return `${address.substring(0, start)}...${address.substring(address.length - end)}`;
};