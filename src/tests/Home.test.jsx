import Home from '@/app/pages/Home'
import { render, screen } from '@testing-library/react'

test('renders the component', () => {
  render(<Home />)
  expect(screen.getByText(/Luca Lapenna/i)).toBeInTheDocument()
})
