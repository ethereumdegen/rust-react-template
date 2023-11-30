// PaginationBar.js
import React from 'react';

const PaginationBar = ({ currentPage, totalPages, onPageChange }) => {
  const handlePageChange = (newPage) => {
    if (newPage >= 1 && newPage <= totalPages) {
      onPageChange(newPage);
    }
  };

  return (
    <div className="flex items-center justify-center my-4">
      <button
        onClick={() => handlePageChange(currentPage - 1)}
        className="px-4 py-2 mr-1 bg-gray-200 text-gray-700 rounded-lg"
        disabled={currentPage === 1}
      >
        Previous
      </button>
      <span className="mx-2 text-lg">
        {currentPage} / {totalPages}
      </span>
      <button
        onClick={() => handlePageChange(currentPage + 1)}
        className="px-4 py-2 ml-1 bg-gray-200 text-gray-700 rounded-lg"
        disabled={currentPage === totalPages}
      >
        Next
      </button>
    </div>
  );
};

export default PaginationBar;