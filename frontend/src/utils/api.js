// Helper untuk handle API responses
export const handleApiError = (error) => {
  if (error.response?.data) {
    return error.response.data;
  }
  return error.message || 'Something went wrong';
};

export const createQueryString = (params) => {
  const searchParams = new URLSearchParams();
  Object.entries(params).forEach(([key, value]) => {
    if (value !== null && value !== undefined) {
      searchParams.append(key, value.toString());
    }
  });
  return searchParams.toString();
};