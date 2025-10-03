import { useNavigate } from 'react-router-dom';
import { createNavigationHandler } from '../utils/navigationUtils';

/**
 * Custom hook that provides a navigation handler function.
 * Eliminates the repetitive pattern of creating navigation handlers in components.
 *
 * @returns A navigation handler function
 */
export const useNavigation = () => {
  const navigate = useNavigate();
  return createNavigationHandler(navigate);
};
